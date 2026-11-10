use crate::{HANGUL_CARDINAL, HANGUL_DIGITS, HANGUL_NUMBERS, HANGUL_NUMBERS_FOR_DECIMAL, NumberToHangulError};

pub struct NumberToHangulOptions {
    pub spacing: bool,
}

/// number_to_hangul은 인자로 받은 input을 숫자로 변환해줍니다.
///
/// ```rust
/// use hangul_core::number_to_hangul;
///
/// assert_eq!(number_to_hangul("77", None).unwrap(), "칠십칠");
/// ```
fn number_to_hangul(input: &str, options: Option<NumberToHangulOptions>) -> Result<String, NumberToHangulError> {
    let input = input.trim();
    let options = options.unwrap();
    
    if input.is_empty() {
        return Err(NumberToHangulError::EmptyInput)
    }

    if input == "Infinity" || input == "+Infinity" {
        return Ok("무한대".to_string());
    }

    if input == "-Infinity" {
        return Ok(if options.spacing {
            "마이너스 무한대"
        } else {
            "마이너스무한대"
        }.to_string());
    }
    
    let (is_negative, number) = if let Some(value) = input.strip_prefix('-') {
        (true, value)
    } else if let Some(value) = input.strip_prefix('+') {
        (false, value)
    } else {
        (false, input)
    };

    if number.is_empty() {
        return Err(NumberToHangulError::InvalidNumber);
    }

    let (integer_part, decimal_part) = match number.split_once('.') {
        Some((integer, decimal)) => {
            if number.matches('.').count() != 1 {
                return Err(NumberToHangulError::InvalidNumber);
            }

            (integer, Some(decimal))
        }
        None => (number, None),
    };

    if integer_part.is_empty() && decimal_part.is_none() {
        return Err(NumberToHangulError::InvalidNumber);
    }

    if integer_part.is_empty() && decimal_part == Some("") {
        return Err(NumberToHangulError::InvalidNumber);
    }

    if !integer_part.bytes().all(|b| b.is_ascii_digit()) {
        return Err(NumberToHangulError::InvalidNumber);
    }

    if let Some(decimal) = decimal_part {
        if !decimal.bytes().all(|b| b.is_ascii_digit()) {
            return Err(NumberToHangulError::InvalidNumber);
        }
    }

    let integer_part = if integer_part.is_empty() {
        "0"
    } else {
        integer_part
    };

    let integer_part = integer_part.trim_start_matches('0');

    let integer_part = if integer_part.is_empty() {
        "0"
    } else {
        integer_part
    };

    if integer_part == "0"
        && decimal_part.map_or(true, |decimal| {
            decimal.bytes().all(|b| b == b'0')
        })
    {
        return Ok("영".to_string());
    }

    let spacing = if options.spacing { " " } else { "" };

    let mut result = integer_to_hangul(integer_part, spacing)?;

    if let Some(decimal) = decimal_part {
        if !decimal.is_empty() {
            result.push_str(spacing);
            result.push_str("점");

            for digit in decimal.bytes() {
                let digit = (digit - b'0') as usize;
                result.push_str(HANGUL_NUMBERS_FOR_DECIMAL[digit]);
            }
        }
    }

    if is_negative {
        result = format!("마이너스{spacing}{result}");
    }

    Ok(result)
}

fn integer_to_hangul(
    number: &str,
    spacing: &str,
) -> Result<String, NumberToHangulError> {
    if number == "0" {
        return Ok("영".to_string());
    }

    let mut parts = Vec::new();

    for (index, chunk) in number.as_bytes().rchunks(4).enumerate() {
        let chunk = std::str::from_utf8(chunk)
            .map_err(|_| NumberToHangulError::InvalidNumber)?;

        let value = chunk
            .parse::<u16>()
            .map_err(|_| NumberToHangulError::InvalidNumber)?;

        if value == 0 {
            continue;
        }

        let unit = HANGUL_DIGITS
            .get(index)
            .ok_or(NumberToHangulError::UnitRangeExceeded)?;

        let korean_number = number_to_korean_up_to_thousand(value);

        parts.push(format!("{korean_number}{unit}"));
    }

    parts.reverse();

    Ok(parts.join(spacing))
}

fn number_to_korean_up_to_thousand(number: u16) -> String {
    let mut parts = Vec::new();
    let mut remaining = number;

    for (index, cardinal) in HANGUL_CARDINAL.iter().enumerate().rev() {
        let divisor = 10_u16.pow(index as u32);
        let digit = remaining / divisor;

        remaining %= divisor;

        if digit == 0 {
            continue;
        }

        if digit == 1 && index > 0 {
            parts.push(cardinal.to_string());
        } else {
            parts.push(format!(
                "{}{}",
                HANGUL_NUMBERS[digit as usize],
                cardinal
            ));
        }
    }

    parts.join("")
}

