const COUNT_MOTOR_LR_POWER_RATIOS: usize = 12;
static MOTOR_LR_POWER_RATIOS: [(u8, f32); COUNT_MOTOR_LR_POWER_RATIOS] = [
    // (targer_power_level: i32, left_right_turn_ratio: f32)
    (70,	1.00467),
    (80,	0.98837),
    (90,	1.00244),
    (100,	0.99218),
    (110,	0.98535),
    (120,	1.00406),
    (140,	1.00608),
    (160,	0.99909),
    (180,	0.97128),
    (200,	0.9729),
    (225,	0.95067),
    (255,	0.87813),
];

/// For a nominal power level, returns the calibrated (left, right) motor power levels
pub fn get_lr_motor_power(target_power_level: u8) -> (u8, u8) {
    let mut left_power: u8 = 125;
    let mut right_power: u8 = 125;

    for i in 0..COUNT_MOTOR_LR_POWER_RATIOS {
        if target_power_level < MOTOR_LR_POWER_RATIOS[i].0 {
            if i == 0 {
                left_power = MOTOR_LR_POWER_RATIOS[i].0;
                right_power =
                    ((MOTOR_LR_POWER_RATIOS[i].0 as f32) * MOTOR_LR_POWER_RATIOS[i].1) as u8;
            } else {
                let lr_ratio = MOTOR_LR_POWER_RATIOS[i - 1].1
                    + (MOTOR_LR_POWER_RATIOS[i].1 - MOTOR_LR_POWER_RATIOS[i - 1].1)
                        * (target_power_level - MOTOR_LR_POWER_RATIOS[i - 1].0) as f32
                        / (MOTOR_LR_POWER_RATIOS[i].0 - MOTOR_LR_POWER_RATIOS[i - 1].0) as f32;
                left_power = target_power_level;
                right_power = ((target_power_level as f32) * lr_ratio) as u8;
            }
            break;
        } else if i == COUNT_MOTOR_LR_POWER_RATIOS - 1 {
            left_power = MOTOR_LR_POWER_RATIOS[i].0;
            right_power = ((MOTOR_LR_POWER_RATIOS[i].0 as f32) * MOTOR_LR_POWER_RATIOS[i].1) as u8;
        }
    }
    (left_power, right_power)
}
