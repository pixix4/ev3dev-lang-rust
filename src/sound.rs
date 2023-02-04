//! Sound-related functions. It can beep, play wav files, or convert text to
//! speech.
//!
//! Note that all methods of the module spawn system processes and return
//! `std::process::Child` objects. The methods are asynchronous (they return
//! immediately after child process was spawned, without waiting for its
//! completion), but you can call wait() on the returned result.
//!
//! # Examples
//! ```no_run
//! # use ev3dev_lang_rust::Ev3Result;
//! use ev3dev_lang_rust::sound;
//!
//! # fn main() -> Ev3Result<()> {
//! // Play "bark.wav", return immediately:
//! sound::play("bark.wav")?;
//!
//! // Introduce yourself, wait for completion:
//! sound::speak("Hello, I am Robot")?.wait()?;
//! # Ok(())
//! # }
//! ```

use crate::{Ev3Error, Ev3Result};
use std::ffi::OsStr;
use std::process::{Child, Command, Stdio};

/// Call beep command.
///
/// # Example
/// ```no_run
/// # use ev3dev_lang_rust::Ev3Result;
/// use ev3dev_lang_rust::sound;
///
/// # fn main() -> Ev3Result<()> {
/// sound::beep()?.wait()?;
/// # Ok(())
/// # }
/// ```
pub fn beep() -> Ev3Result<Child> {
    Ok(Command::new("/usr/bin/beep")
        .stdout(Stdio::null())
        .spawn()?)
}

/// Call beep command with the provided arguments.
///
/// See `beep man page`_ and google `linux beep music`_ for inspiration.
/// * `beep man page`: <https://linux.die.net/man/1/beep>
/// * `linux beep music`: <https://www.google.com/search?q=linux+beep+music>
///
/// # Example
/// ```no_run
/// # use ev3dev_lang_rust::Ev3Result;
/// use ev3dev_lang_rust::sound;
///
/// # fn main() -> Ev3Result<()> {
/// sound::beep_args(&[""])?.wait()?;
/// # Ok(())
/// # }
/// ```
pub fn beep_args<I, S>(args: I) -> Ev3Result<Child>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Ok(Command::new("/usr/bin/beep")
        .args(args)
        .stdout(Stdio::null())
        .spawn()?)
}

/// Play tone sequence. The tone_sequence parameter is a list of tuples,
/// where each tuple contains up to three numbers. The first number is
/// frequency in Hz, the second is duration in milliseconds, and the third
/// is delay in milliseconds between this and the next tone in the
/// sequence.
///
/// # Example
/// ```no_run
/// # use ev3dev_lang_rust::Ev3Result;
/// use ev3dev_lang_rust::sound;
///
/// # fn main() -> Ev3Result<()> {
/// sound::tone(466.0, 500)?.wait()?;
/// # Ok(())
/// # }
pub fn tone(frequency: f32, duration: i32) -> Ev3Result<Child> {
    beep_args(vec![format!("-f {frequency}"), format!("-l {duration}")])
}

/// Play tone sequence. The tone_sequence parameter is a list of tuples,
/// where each tuple contains up to three numbers. The first number is
/// frequency in Hz, the second is duration in milliseconds, and the third
/// is delay in milliseconds between this and the next tone in the
/// sequence.
///
/// # Example
/// ```no_run
/// # use ev3dev_lang_rust::Ev3Result;
/// use ev3dev_lang_rust::sound;
///
/// # fn main() -> Ev3Result<()> {
/// sound::tone_sequence(
///     &[
///         (392.00, 350, 100), (392.00, 350, 100), (392.00, 350, 100), (311.1, 250, 100),
///         (466.20, 025, 100), (392.00, 350, 100), (311.10, 250, 100), (466.2, 025, 100),
///         (392.00, 700, 100), (587.32, 350, 100), (587.32, 350, 100),
///         (587.32, 350, 100), (622.26, 250, 100), (466.20, 025, 100),
///         (369.99, 350, 100), (311.10, 250, 100), (466.20, 025, 100), (392.00, 700, 100),
///         (784.00, 350, 100), (392.00, 250, 100), (392.00, 025, 100), (784.00, 350, 100),
///         (739.98, 250, 100), (698.46, 025, 100), (659.26, 025, 100),
///         (622.26, 025, 100), (659.26, 050, 400), (415.30, 025, 200), (554.36, 350, 100),
///         (523.25, 250, 100), (493.88, 025, 100), (466.16, 025, 100), (440.00, 025, 100),
///         (466.16, 050, 400), (311.13, 025, 200), (369.99, 350, 100),
///         (311.13, 250, 100), (392.00, 025, 100), (466.16, 350, 100), (392.00, 250, 100),
///         (466.16, 025, 100), (587.32, 700, 100), (784.00, 350, 100), (392.00, 250, 100),
///         (392.00, 025, 100), (784.00, 350, 100), (739.98, 250, 100), (698.46, 025, 100),
///         (659.26, 025, 100), (622.26, 025, 100), (659.26, 050, 400), (415.30, 025, 200),
///         (554.36, 350, 100), (523.25, 250, 100), (493.88, 025, 100),
///         (466.16, 025, 100), (440.00, 025, 100), (466.16, 050, 400), (311.13, 025, 200),
///         (392.00, 350, 100), (311.13, 250, 100), (466.16, 025, 100),
///         (392.00, 300, 150), (311.13, 250, 100), (466.16, 025, 100), (392.00, 700, 0)
///     ]
/// )?.wait()?;
/// # Ok(())
/// # }
pub fn tone_sequence(sequence: &[(f32, i32, i32)]) -> Ev3Result<Child> {
    let tones: Vec<String> = sequence
        .iter()
        .map(|(frequency, duration, delay)| {
            vec![
                format!("-f {frequency}"),
                format!("-l {duration}"),
                format!("-D {delay}"),
            ]
        })
        .collect::<Vec<Vec<String>>>()
        .join(&["-n".to_owned()][..]);

    beep_args(tones)
}

/// Play wav file
pub fn play(wav_file: &str) -> Ev3Result<Child> {
    Ok(Command::new("/usr/bin/aplay")
        .arg("-q")
        .arg(wav_file)
        .stdout(Stdio::null())
        .spawn()?)
}

/// Speak the given text aloud.
pub fn speak(text: &str) -> Ev3Result<Child> {
    let espeak = Command::new("/usr/bin/espeak")
        .args(["--stdout", "-a", "200", "-s", "130", text])
        .stdout(Stdio::piped())
        .spawn()?;

    Ok(Command::new("/usr/bin/aplay")
        .arg("-q")
        .stdin(espeak.stdout.ok_or(Ev3Error::InternalError {
            msg: "`espeak` pipe to `aplay` could not be created!".to_owned(),
        })?)
        .stdout(Stdio::null())
        .spawn()?)
}

/// Get the main channel name or 'Playback' if not available.
fn get_channels() -> Ev3Result<Vec<String>> {
    let out = String::from_utf8(
        Command::new("/usr/bin/amixer")
            .arg("scontrols")
            .output()?
            .stdout,
    )?;

    let mut channels: Vec<String> = out
        .split('\n')
        .filter_map(|line| {
            let vol_start = line.find('\'').unwrap_or(0) + 1;
            let vol_end = line.rfind('\'').unwrap_or(1);

            if vol_start >= vol_end {
                None
            } else {
                Some(line[vol_start..vol_end].to_owned())
            }
        })
        .collect();

    if channels.is_empty() {
        channels.push("Playback".to_owned());
    }

    Ok(channels)
}

/// Sets the sound volume to the given percentage [0-100] by calling
/// `amixer -q set <channel> <pct>%`.
pub fn set_volume_channel(volume: i32, channel: &str) -> Ev3Result<()> {
    Command::new("/usr/bin/amixer")
        .args(["-q", "set", channel, &format!("{volume}%")])
        .stdout(Stdio::null())
        .spawn()?
        .wait()?;

    Ok(())
}

/// Sets the sound volume to the given percentage [0-100] by calling
/// `amixer -q set <channel> <pct>%`.
/// It tries to determine the default channel
/// by running `amixer scontrols`. If that fails as well, it uses the
/// `Playback` channel, as that is the only channel on the EV3.
pub fn set_volume(volume: i32) -> Ev3Result<()> {
    for channel in get_channels()? {
        set_volume_channel(volume, &channel)?;
    }
    Ok(())
}

/// Gets the current sound volume by parsing the output of
/// `amixer get <channel>`.
pub fn get_volume_channel(channel: &str) -> Ev3Result<i32> {
    let out = String::from_utf8(
        Command::new("/usr/bin/amixer")
            .args(["get", channel])
            .output()?
            .stdout,
    )?;

    let vol_start = out.find('[').unwrap_or(0) + 1;
    let vol_end = out.find("%]").unwrap_or(1);
    let vol = &out[vol_start..vol_end].parse::<i32>()?;

    Ok(*vol)
}

/// Gets the current sound volume by parsing the output of
/// `amixer get <channel>`.
/// It tries to determine the default channel
/// by running `amixer scontrols`. If that fails as well, it uses the
/// `Playback` channel, as that is the only channel on the EV3.
pub fn get_volume() -> Ev3Result<i32> {
    get_volume_channel(&get_channels()?[0])
}
