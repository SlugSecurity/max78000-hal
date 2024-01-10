//! Timers peripheral API...
//!
//! User Guide: 
//! Section 19
//! 
//! Introduction:
//! The MAX78000 includes multiple 32-bit and dual 16-bit, reloadable timers.
//! Instances denoted as LPTMR, shown in Table 19-1, are configurable to operate 
//! in any of the low-power modes and wake the device from the low-power modes to ACTIVE.
//!
//! Important Notation:
//! TMR = TMR0 
//! TMR1 = TMR1
//! TMR2 = TMR2
//! TMR3 = TMR3
//! LPTMR0 = TMR4
//! LPTMR1 = TMR5

use core::mem;

use max78000::{TMR, TMR1, TMR2, TMR3, TMR4, TMR5};

//! TRM0
pub struct timer0 {
    tmr: TMR,
}

//! TRM1
pub struct timer1 {
    tmr: TMR1,
}

//! TRM2
pub struct timer2 {
    tmr: TMR2,
}

//! TRM3
pub struct timer3 {
    tmr: TMR3,
}

//! TRM4
pub struct timer4 {
    tmr: TMR4,
}

//! TRM5
pub struct timer5 {
    tmr: TMR5,
}

//! Timer methods
impl timer0 {
    //! new() - Constructor
    pub fn new(tmr:TMR) -> Self {
        Self { tmr }
    }

    //! Clock and Timer control functions
    //! tmr_disable() - The timer peripheral should be disabled while changing any of the registers in the peripheral. This function does just that.
    fn tmr_disable(){
        //! a. Clear TMRn_CTRL0.en to 0 to disable the timer.
        //! b. Read the TMRn_CTRL1.clken field until it returns 0, confirming the timer peripheral is disabled.
    }

    //! tmr_enable() - Enable the timer peripheral
    fn tmr_enable(){
        //! a. Set TMRn_CTRL0.en to 1 to enable the timer.
        //! b. Read the TMRn_CTRL0.clken field until it returns 1 to confirm the timer is enabled.
    }

    //! clk_disable() - Disables the timer clock source
    fn clk_disable(){
        //! a. Set the TMRn_CTRL0.clken field to 0 to disable the timer's clock source.
        //! b. Read the TMRn_CTRL1.clkrdy field until it returns 0, confirming the timer clock source is disabled.
    }

    //! clk_enable() - Enables the timer clock source
    fn clk_enable(){
        //! a. Set the TMRn_CTRL0.clken field to 1 to enable the timer's clock source.
        //! b. Read the TMRn_CTRL1.clkrdy field until it returns 1, confirming the timer clock source is enabled.
    }

    //! Operating Mode functions
    //! oneShot() - In one-shot mode, the timer peripheral increments the timer's TMRn_CNT.count field until it reaches the timer's
    //!              TMRn_CMP.compare field, and the timer is then disabled. If the timer's output is enabled, the output signal is driven active
    //!              for one timer clock cycle. One-shot mode provides exactly one timer period and is automatically disabled.
    fn oneShot(){
        //! Configure the timer for one-shot mode by performing the following steps:
        //! 1. Disable the timer peripheral and set the timer clock source as described in Timer Clock Sources.
        //! 2. Set the TMRn_CTRL0.mode field to 0 to select one-shot mode.
        //! 3. Set the TMRn_CTRL0.pres field to set the prescaler for the required timer frequency.
        //! 4. If using the timer output function:
        //! a. Set TMRn_CTRL0.pol to match the desired inactive state.
        //! b. Configure the GPIO electrical characteristics as desired.
        //! c. Select the correct alternate function mode for the timer output pin.
        //! 5. Or, if using the inverted timer output function:
        //! a. Set TMRn_CTRL0.pol to match the desired inactive state.
        //! b. Configure the GPIO electrical characteristics as desired.
        //! c. Select the correct alternate function mode for the inverted timer output pin.
        //! 6. If using the timer interrupt, enable the corresponding field in the TMRn_CTRL1 register.
        //! 7. Write the compare value to the TMRn_CMP.compare field.
        //! 8. If desired, write an initial value to the TMRn_CNT.count field.
        //! a. This affects only the first period; subsequent timer periods always reset the TMRn_CNT.count field to 0x0000 0001.
        //! 9. Enable the timer peripheral as described in Timer Clock Sources.
    }

    //! continous() - In continuous mode, the TMRn_CNT.count field increments until it matches the TMRn_CMP.compare field; the
    //!               TMRn_CNT.count field is then set to 0x0000 0001, and the count continues to increment. Optionally, application software
    //!               can configure continuous mode to toggle the timer output pin at the end of each timer period. A continuous mode timer
    //!               period ends when the timer count field reaches the timer compare field (TMRn_CNT.count = TMRn_CMP.compare).
    fn continuous(){
        //! Configure the timer for continuous mode by performing the following steps:
        //! 1. Disable the timer peripheral and set the timer clock as described in Timer Clock Sources.
        //! 2. Set the TMRn_CTRL0.mode field to 1 to select continuous mode.
        //! 3. Set the TMRn_CTRL0.pres field to set the prescaler that determines the timer frequency.
        //! 4. If using the timer output function:
        //!     a. Set TMRn_CTRL0.pol to match the desired (inactive) state.
        //!     b. Configure the GPIO electrical characteristics as desired.
        //!     c. Select the correct alternate function mode for the timer output pin.
        //! 5. Or, if using the inverted timer output function:
        //!     a. Set TMRn_CTRL0.pol to match the desired (inactive) state.
        //!     b. Configure the GPIO electrical characteristics as desired.
        //!     c. Select the correct alternate function mode for the inverted timer output pin.
        //! 6. If using the timer interrupt, enable the corresponding field in the TMRn_CTRL1 register.
        //! 7. Write the compare value to the TMRn_CMP.compare field.
        //! 8. If desired, write an initial value to the TMRn_CNT.count field.
        //! a. This affects only the first period; subsequent timer periods always reset the TMRn_CNT.count field to 0x0000 0001.
        //! 9. Enable the timer peripheral as described in Timer Clock Sources
    }

    //! counter() - In counter mode, the timer peripheral increments the TMRn_CNT.count each time a transition occurs on the timer input
    //!             signal. The transition must be greater than 4 × 𝑃𝑃𝑃𝑃𝑃𝑃𝑃𝑃 for a count to occur. When the TMRn_CNT.count reaches
    //!             the TMRn_CMP.compare field, the hardware automatically sets the interrupt bit to 1 (TMRn_INTFL.irq), sets the
    //!             TMRn_CNT.count field to 0x0000 0001, and continues incrementing. The timer can be configured to increment on either
    //!             the timer's input signal's rising edge or falling edge, but not both. Use the TMRn_CTRL0.pol_ field to select which edge is
    //!             used for the timer's input signal count.
    fn counter(){
        //! Configure the timer for counter mode by performing the following:
        //! 1. Disable the timer peripheral as described in Timer Clock Sources.
        //! 2. If desired, change the timer clock source as described in Timer Clock Sources.
        //! 3. Set TMRn_CTRL0.mode 0x2 to select Counter mode.
        //! 4. Configure the timer input function:
        //!     a. Set TMRn_CTRL0.pol to match the desired (inactive) state.
        //!     b. Configure the GPIO electrical characteristics as desired.
        //!     c. Set TMRn_CTRL1.outen_a and TMRn_CTRL1.outben to the values shown in the Operating Modes section.
        //!     d. Select the correct alternate function mode for the timer input pin.
        //! 5. Write the compare value to TMRn_CMP.compare.
        //! 6. If desired, write an initial value to TMRn_CNT.count. This affects only the first period; subsequent timer periods always reset TMRn_CNT.count = 0x0000 0001.
        //! 7. Enable the timer peripheral as described in Timer Clock Sources.
    }

    //! pwm() - In PWM mode, the timer sends a PWM output using the timer's output signal. The timer first counts up to the match value
    //!         stored in the TMRn_PWM.pwm register. At the end of the cycle, where the TMRn_CNT.count value matches the
    //!         TMRn_PWM.pwm, the timer output signal toggles state. The timer continues counting until it reaches the
    //!         TMRn_CMP.compare value.
    //! 
    //!         The timer period ends on the rising edge of fCNT_CLK following TMRn_CNT.count = TMRn_CMP.compare
    fn pwm(){
        //! Complete the following steps to configure a timer for PWM mode and initiate the PWM operation:
        //! 1. Disable the timer peripheral as described in Timer Clock Sources.
        //! 2. If desired, change the timer clock source as described in Timer Clock Sources.
        //! 3. Set the TMRn_CTRL0.mode field to 3 to select PWM mode.
        //! 4. Set the TMRn_CTRL0.pres field to set the prescaler that determines the timer frequency.
        //! 5. Configure the pin as a timer input and configure the electrical characteristics as needed.
        //! 6. Set TMRn_CTRL0.pol to match the desired initial (inactive) state.
        //! 7. Set TMRn_CTRL0.pol to select the initial logic level (high or low) and PWM transition state for the timer's output.
        //! 8. Set TMRn_CNT.count initial value if desired.
        //!     a. The initial TMRn_CNT.count value only affects the initial period in PWM mode, with subsequent periods always setting TMRn_CNT.count to 0x0000 0001.
        //! 9. Set the TMRn_PWM value to the transition period count.
        //! 10. Set the TMRn_CMP.compare value for the PWM second transition period. Note: TMRn_CMP.compare must be greater than the TMRn_PWM value.
        //! 11. If using the timer interrupt, set the interrupt priority and enable the interrupt.
        //! 12. Enable the timer peripheral as described in Timer Clock Sources.
    }

    //! capture() - Capture mode is used to measure the time between software-determined events. The timer starts incrementing the timer's 
    //!             count field until a transition occurs on the timer's input pin or a rollover event occurs. A capture event is triggered by the
    //!             hardware when the timer's input pin transitions state. Equation 19-9 shows the formula for calculating the capture event's
    //!             elapsed time.
    //! 
    //!             If a capture event does not occur before the timer's count value reaching the timer's compare value (TMRn_CNT.count = TMRn_CMP.compare), a rollover event occurs. The capture event and the rollover event set the timer's
    //!             interrupt flag (TMRn_INTFL.irq = 1) resulting in an interrupt if the timer's interrupt is enabled.
    //! 
    //!             A capture event can occur before or after a rollover event. The software must track the number of rollover events that
    //!             occur before a capture event to determine the elapsed time of the capture event. When a capture event occurs, the
    //!             software should reset the count of rollover events.
    fn capture(){
        //! Configure the timer for capture mode by doing the following:
        //! 1. Disable the timer peripheral as described in Timer Clock Sources.
        //! 2. If desired, change the timer clock source as described in Timer Clock Sources.
        //! 3. Set TMRn_CTRL0.mode to 4 to select capture mode.
        //! 4. Configure the timer input function:
        //!     a. Set TMRn_CTRL0.pol to match the desired inactive state.
        //!     b. Configure the GPIO electrical characteristics as desired.
        //!     c. Select the correct alternate function mode for the timer input pin.
        //! 5. Write the initial value to TMRn_CNT.count, if desired.
        //!     a. This affects only the first period; subsequent timer periods always reset TMRn_CNT.count = 0x0000 0001.
        //! 6 Write the compare value to the TMRn_CMP.compare field.
        //! 7. Select the capture event by setting TMRn_CTRL1.capeventsel.
        //! 8. Enable the timer peripheral as described in Timer Clock Sources.
    }

    //! compare() - In compare mode, the timer peripheral increments continually from 0x0000 0000 (after the first timer period) to the
    //!             maximum value of the 32- or 16-bit mode, then rolls over to 0x0000 0000 and continues incrementing. The end of timer
    //!             period event occurs when the timer value matches the compare value, but the timer continues to increment until the count
    //!             reaches 0xFFFF FFFF. The timer counter then rolls over and continues counting from 0x0000 0000.
    //! 
    //!             The timer period ends on the timer clock following TMRn_CNT.count = TMRn_CMP.compare.
    fn compare(){
        //! Configure the timer for compare mode by doing the following:
        //! 1. Disable the timer peripheral as described in Timer Clock Sources.
        //! 2. If desired, change the timer clock source as described in Timer Clock Sources.
        //! 3. Set TMRn_CTRL0.mode to 5 to select Compare mode.
        //! 4. Set TMRn_CTRL0.pres to set the prescaler that determines the timer frequency.
        //! 5. If using the timer output function:
        //!     a. Set TMRn_CTRL0.pol to match the desired (inactive) state.
        //!     b. Configure the GPIO electrical characteristics as desired.
        //!     c. Select the correct alternate function mode for the timer output pin.
        //! 6. If using the inverted timer output function:
        //!     a. Set TMRn_CTRL0.pol to match the desired (inactive) state.
        //!     b. Configure the GPIO electrical characteristics as desired.
        //!     c. Select the correct alternate function mode for the inverted timer output pin.
        //! 7. If using the timer interrupt, enable the corresponding field in the TMRn_CTRL1 register.
        //! 8. Write the compare value to TMRn_CMP.compare.
        //! 9. If desired, write an initial value to TMRn_CNT.count.
        //!     a. This affects only the first period; subsequent timer periods always reset TMRn_CNT.count = 0x0000 0001.
        //! 10. Enable the timer peripheral as described in Timer Clock Sources.
    }

    //! gated() - Gated mode is similar to continuous mode, except that TMRn_CNT.count only increments when the timer input signal is in
    //!           its active state.
    //! 
    //!           The timer period ends on the timer clock following TMRn_CNT.count = TMRn_CMP.compare.
    fn gated(){
        //! Configure the timer for gated mode by performing the following steps:
        //! 1. Disable the timer peripheral as described in Timer Clock Sources.
        //! 2. If desired, change the timer clock source as described in Timer Clock Sources.
        //! 3. Set TMRn_CTRL0.mode to 6 to select gated mode.
        //! 4. Configure the timer input function:
        //!     a. Set TMRn_CTRL0.pol to match the desired inactive state.
        //!     b. Configure the GPIO electrical characteristics as desired.
        //!     c. Select the correct alternate function mode for the timer input pin.
        //! 5. If desired, write an initial value to the TMRn_CNT.count field.
        //!     a. This only effects the first period; subsequent timer periods always reset TMRn_CNT.count = 0x0000 0001.
        //! 6 Write the compare value to TMRn_CMP.compare.
        //! 7. Enable the timer peripheral as described in Timer Clock Sources.
    }

    //! captureCompare() - In capture/compare mode, the timer starts counting after the first external timer input transition occurs. The transition, a
    //!                    rising edge or falling edge on the timer's input signal, is set using the TMRn_CTRL0.pol bit.
    //! 
    //!                    After the first transition of the timer input signal, each subsequent transition captures the TMRn_CNT.count value, writing it
    //!                    to the TMRn_PWM.pwm register (capture event). When a capture event occurs, a timer interrupt is generated, the TMRn_CNT.count value is reset to 0x0000_0001, and the timer resumes counting.
    //! 
    //!                    If no capture event occurs, the timer counts up to TMRn_CMP.compare. At the end of the cycle, where the
    //!                    TMRn_CNT.count equals the TMRn_CMP.compare, a timer interrupt is generated, the TMRn_CNT.count value is reset to 0x0000 0001, and the timer resumes counting.
    //! 
    //!                    The timer period ends when the selected transition occurs on the timer pin or the clock cycle following TMRn_CNT.count = TMRn_CMP.compare.
    fn captureCompare(){
        //! Configure the timer for capture/compare mode by doing the following:
        //! 1. Disable the timer peripheral as described in Timer Clock Sources.
        //! 2. If desired, change the timer clock source as described in Timer Clock Sources.
        //! 3. Set TMRn_CTRL0.mode to 7 to select Capture/Compare mode.
        //! 4. Configure the timer input function:
        //!     a. Set TMRn_CTRL0.pol to select the positive edge (TMRn_CTRL0.pol = 1) or negative edge (TMRn_CTRL0.pol = 0) transition to cause the capture event.
        //!     b. Configure the GPIO electrical characteristics as desired.
        //!     c. Select the correct alternate function mode for the timer input pin.
        //! 5. If desired, write an initial value to the TMRn_CNT.count field.
        //!     a. This effects only the first period; subsequent timer periods always reset TMRn_CNT.count = 0x0000 0001.
        //! 6 Write the compare value to TMRn_CMP.compare.
        //! 7. Enable the timer peripheral as described in Timer Clock Sources.
    }

    //! clk_cfg() - Function to configure the timer's clock source
    fn clk_cfg() {
        //! 1. Disable the timer peripheral
        //! 2. Set TMRn_CTRL1.clksel to the new desired clock source.
        //! 3. Configure the timer for the desired operating mode. See Operating Modes for details on mode configuration.
        //! 4. Enable the timer clock source.
        //! 5. Enable the timer.
    }
}