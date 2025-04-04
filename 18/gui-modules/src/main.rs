pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut double = false;

    for c in cc_number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            if double {
                let double_digit = digit * 2;
                sum +=
                    if double_digit > 9 { double_digit - 9 } else { double_digit };
            } else {
                sum += digit;
            }
            double = !double;
        } else {
            continue;
        }
    }

    sum % 10 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }
}

// Дополнительные тесты
#[cfg(test)]
mod test_new {
    use super::*;

    // Тесты для проверки "хороших" номеров для алгоритма Луна
    #[test]
    fn test_good_numbers(){
        assert!(luhn("4-5-3-9-1-4-8-8-0-3-4-3-6-4-6-7")); // Кредитная карточка
        assert!(luhn("ЭПД 110 494-2")); // Номер локомотива
        assert!(luhn("000000000000049927398716")); // Нули слева не учитываются, так как цифры рассматриваются в обратном порядке
    }

    // Тесты для проверки "не совсем хороших" номеров для алгоритма Луна
    #[test]
    fn test_bad_numbers(){
        assert!(!luhn("1234 5678 1234 5678")); // Снова кредитная карточка
        assert!(!luhn("4--5()3--91$4%%88-0343 6**4-68")); // Ещё одна кредитная карточка
        assert!(!luhn("Дата основания МГТУ - 14 июля (7 месяца календаря) года 1830")); // Дата основания МГТУ
    }

    // Тесты для проверки необычных вариантов
    #[test]
    fn test_unusual_numbers(){
        assert!(luhn("0000000000000000000000000000000000")); // Если все цифры - 0
        assert!(!luhn("1")); // Одиночная цифра, не равная 0 - провал
        assert!(luhn("")); // А где цифры? Почему пустая строка проходит тест?!
        assert!(luhn("Где карта, Билли?!")); // Ещё раз - ГДЕ цифры, Билли?!
        assert!(!luhn("10")); // Небольшие числа тоже не сгодятся
        assert!(luhn("123456789098765432101234")); // А вот большие числа в определенной последовательности работают
    }
}

fn main() {
    // Просто, чтобы было main - компилятор не будет Warning-и кидать
}



//fn main() {
//    println!("Hello, world!");
//}
