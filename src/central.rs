#![no_std]
#![no_main]

#[macro_use]
mod macros;
mod keymap;
mod vial;

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::{
    self as _, bind_interrupts,
    gpio::{AnyPin, Input, Output},
    interrupt::{self, InterruptExt, Priority},
    peripherals::{self, SAADC},
    saadc::{self, AnyInput, Input as _, Saadc},
    usb::{self, vbus_detect::SoftwareVbusDetect, Driver},
    Peripheral,
};
use panic_probe as _;
use rmk::{
    ble::SOFTWARE_VBUS,
    channel::EVENT_CHANNEL,
    config::{
        BleBatteryConfig, ControllerConfig, KeyboardUsbConfig, RmkConfig, StorageConfig, VialConfig,
    },
    debounce::default_debouncer::DefaultDebouncer,
    futures::future::{join, join4},
    initialize_keymap_and_storage, initialize_nrf_sd_and_flash,
    input_device::{
        adc::{EventType, NrfAdc},
        battery::BatteryProcessor,
        joystick::JoystickProcessor,
        Runnable,
    },
    keyboard::Keyboard,
    light::LightController,
    run_devices, run_processor_chain, run_rmk,
    split::central::{run_peripheral_manager, CentralDirectPinMatrix},
};

use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

bind_interrupts!(struct Irqs {
    USBD => usb::InterruptHandler<peripherals::USBD>;
    SAADC => saadc::InterruptHandler;
});

/// Initializes the SAADC peripheral in single-ended mode on the given pin.
fn init_adc<const N: usize>(adc_pin: [AnyInput; N], adc: SAADC) -> Saadc<'static, N> {
    // Then we initialize the ADC. We are only using one channel in this example.
    let config = saadc::Config::default();

    let channel_cfg: [saadc::ChannelConfig; N] = core::array::from_fn(|i| {
        saadc::ChannelConfig::single_ended(unsafe { adc_pin[i].clone_unchecked() }.degrade_saadc())
    });

    interrupt::SAADC.set_priority(interrupt::Priority::P3);
    let saadc = saadc::Saadc::new(adc, Irqs, config, channel_cfg);
    saadc
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello NRF BLE!");
    let mut nrf_config = embassy_nrf::config::Config::default();
    nrf_config.gpiote_interrupt_priority = Priority::P3;
    nrf_config.time_interrupt_priority = Priority::P3;
    interrupt::USBD.set_priority(interrupt::Priority::P2);
    interrupt::CLOCK_POWER.set_priority(interrupt::Priority::P2);
    let p = embassy_nrf::init(nrf_config);

    // Usb config
    let software_vbus = SOFTWARE_VBUS.get_or_init(|| SoftwareVbusDetect::new(true, false));
    let driver = Driver::new(p.USBD, Irqs, software_vbus);

    // Initialize the ADC. We are only using one channel for detecting battery level

    // Wait for ADC calibration.

    // Keyboard config
    let keyboard_usb_config = KeyboardUsbConfig {
        vid: 0x4c4b,
        pid: 0x4643,
        manufacturer: "awe",
        product_name: "RMK Keyboard",
        serial_number: "vial:f64c2b3c:000001",
    };
    let vial_config = VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF);
    let ble_battery_config = BleBatteryConfig::new(None, true, None, false);
    let storage_config = StorageConfig {
        start_addr: 0,
        num_sectors: 6,
        ..Default::default()
    };
    let rmk_config = RmkConfig {
        usb_config: keyboard_usb_config,
        vial_config,
        ble_battery_config,
        storage_config,
        ..Default::default()
    };

    let direct_pins = config_matrix_direct_pins_nrf! {
        peripherals: p,
        direct_pins:
            [
                [P0_22, P0_17, P0_08, P0_24, P0_20],
                [P1_13, P1_15, P0_02, P1_00, P0_10],
                [P1_08, P1_02, P0_12, P0_07, P1_11],
                [_, _, P0_11, P0_09, P1_06],
            ]
    };

    // Initialize the Softdevice and flash
    let central_addr = [0x18, 0xe2, 0x21, 0x80, 0xc0, 0xc7];
    let peripheral_addr = [0x7e, 0xfe, 0x73, 0x9e, 0x66, 0xe3];
    let (sd, flash) = initialize_nrf_sd_and_flash(
        rmk_config.usb_config.product_name,
        spawner,
        Some(central_addr),
    );

    // Initialize the storage and keymap
    let mut default_keymap = keymap::get_default_keymap();
    let (keymap, storage) = initialize_keymap_and_storage(
        &mut default_keymap,
        flash,
        rmk_config.storage_config,
        rmk_config.behavior_config.clone(),
    )
    .await;

    // Initialize the matrix + keyboard
    let debouncer = DefaultDebouncer::<5, 4>::new();
    let mut matrix = CentralDirectPinMatrix::<_, _, 0, 0, 4, 5, { keymap::SIZE }>::new(
        direct_pins,
        debouncer,
        true,
    );
    let mut keyboard = Keyboard::new(&keymap, rmk_config.behavior_config.clone());

    // Initialize the light controller
    let light_controller: LightController<Output> =
        LightController::new(ControllerConfig::default().light_config);

    let saadc = init_adc(
        [saadc::VddhDiv5Input.into(), p.P0_31.into(), p.P0_29.into()],
        p.SAADC,
    );
    saadc.calibrate().await;
    let mut adc_dev = NrfAdc::new(saadc, [EventType::Battery, EventType::Joystick(2)], 20);
    let mut batt_proc = BatteryProcessor::new(1, 5, &keymap);
    let mut joy_proc = JoystickProcessor::new([[80, 0], [0, 80]], [29130, 29365], 6, &keymap);

    // Start
    join4(
        run_devices! (
            (matrix, adc_dev) => EVENT_CHANNEL,
        ),
        run_processor_chain! {
            EVENT_CHANNEL => [joy_proc, batt_proc],
        },
        keyboard.run(),
        join(
            run_peripheral_manager::<4, 5, 4, 0>(0, peripheral_addr),
            run_rmk(&keymap, driver, storage, light_controller, rmk_config, sd),
        ),
    )
    .await;
}
