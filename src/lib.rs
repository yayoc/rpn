use std::error::Error;
use std::io;

pub struct RPN {
    pos: usize,
    input: String,
}

impl RPN {
    fn new(input: String) -> Self {
        RPN { input, pos: 0 }
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.peek_char().unwrap()) {
            result.push(self.consume_char())
        }
        result
    }

    fn consume_num(&mut self) -> String {
        self.consume_while(|c| c.is_digit(10))
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    fn parse(&mut self) -> Result<Vec<f64>, io::Error> {
        let mut nums = vec![];
        while !self.eof() {
            self.consume_whitespace();
            let peek_char = match self.peek_char() {
                Some(c) => c,
                None => {
                    return Ok(nums);
                }
            };

            if peek_char.is_digit(10) {
                let num = self.consume_num();
                nums.push(num.parse().unwrap());
                continue;
            }

            let l2 = nums.pop().ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Found invalid input",
            ))?;
            let l1 = nums.pop().ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Found invalid input",
            ))?;

            match peek_char {
                '+' => nums.push(l1 + l2),
                '-' => nums.push(l1 - l2),
                '*' => nums.push(l1 * l2),
                '/' => nums.push(l1 / l2),
                '%' => nums.push(l1 % l2),
                _ => {}
            }
            self.consume_char();
        }
        Ok(nums)
    }

    pub fn calc(input: String) -> Result<f64, io::Error> {
        let nums = RPN::new(input).parse()?;
        if nums.len() == 1 {
            Ok(nums[0])
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Input syntax could be wrong.",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc() {
        assert_eq!(RPN::calc(String::from("1 2 +")).unwrap(), 3.0);
        assert_eq!(RPN::calc(String::from("1 2 -")).unwrap(), -1.0);
        assert_eq!(RPN::calc(String::from("100 1 +")).unwrap(), 101.0);
        assert_eq!(
            RPN::calc(String::from("15 7 1 1 + - / 3 * 2 1 1 + + -")).unwrap(),
            5.0
        );
        assert_eq!(RPN::calc(String::from("1 2 /")).unwrap(), 0.5);
        assert!(RPN::calc(String::from("15 7")).is_err());
        assert!(RPN::calc(String::from("a b +")).is_err());
    }
}
