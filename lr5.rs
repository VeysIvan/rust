use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

macro_rules! input {($type:expr) => {{
        let mut it = String::new();
        loop {
            it.clear();
            io::stdin().read_line(&mut it).expect("Failed to read input");
            if ($type == "string") {break}
            if (($type == "int") && (it.trim().parse::<i16>().is_ok())) {break}
            else {println!("Ошибка формата. Введите число:");}
        }
        it.trim().parse().expect("please give me correct string number!")
}}}


#[derive(Debug)]
/// Событие лифта на которое должен реагировать контроллер.
enum Event {
    OpenedDoors,
    ClosedDoors,
    TravelTask,
    TravelTaskDown,
    TravelTaskUp,
    TravelSuccess
}

/// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}


struct Car {
    floor: i16, 
    direction: Direction,
    status: Event
}

/// Кабина приехала на заданный этаж.
fn car_arrived(floor: i16) -> Event {
    println!("Кабина приехала на этаж {}", floor);
    car_door_opened();
    car_door_closed();
    return Event::TravelSuccess;
}

/// Двери кабины открыты.
fn car_door_opened() -> Event {
    println!("Двери кабины открыты");
    return Event::OpenedDoors;
}

/// Двери кабины закрыты.
fn car_door_closed() -> Event {
    println!("Двери кабины закрыты");
    return Event::ClosedDoors;
}

/// Кнопка вызова лифта нажата на заданном этаже.
fn lobby_call_button_pressed(floor: i16, dir: Direction) -> Event {
    if (matches!(dir, Direction::Up)) {
        return Event::TravelTaskUp;
    } else {
        return Event::TravelTaskDown;
    }
}

/// Кнопка этажа нажата в кабине лифта.
fn car_floor_button_pressed(floor: i16, liftTasks: &mut Vec<i16>) -> Event {
    liftTasks.push(floor);
    return Event::TravelTask;
}
fn time() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards");
    let in_ms = since_the_epoch.as_secs();
    return in_ms;
}
fn statusLift(personFloor: i16, liftCar: &mut Car, direction: Direction, liftTasks: &mut Vec<i16>) {
    
    if personFloor == liftCar.floor {
        println!("Лифт на вашем этаже");
        if !matches!(liftCar.status, Event::OpenedDoors) {
            liftCar.status = car_door_opened();
        }
        //liftCar.direction = direction;
        car_door_closed();
    } else {
        println!("Ожидайте, пока лифт на этаже {}", liftCar.floor);
        liftCar.status = lobby_call_button_pressed(liftCar.floor, direction);
        car_floor_button_pressed(personFloor, liftTasks);
    }
}



fn main() {
    let mut done = false;
    let mut action: i16;
    let mut message: String;
    let mut timeWas: u64 = time();
    let mut liftTasks = Vec::new();
    let mut timeToReachFloor: u64;
    let mut timeDiff: u64 = 0;
    let mut personFloor: i16 = 1;
    let mut personDestination: i16 = -100;
    let mut tasksStatus: usize = 0;
    
    let mut liftCar = Car { floor: 1, direction: Direction::Up, status: Event::ClosedDoors };
    while !done {
        let timeNow = time();
        //timeWas = timeNow;
        
        if (tasksStatus == 0) {
            timeWas = timeNow;
        }
        if (liftTasks.len() != 0) && (tasksStatus > 0) {
            timeDiff += timeNow - timeWas;
            
            let mut prevFloor = liftCar.floor;
            let mut liftReachedFloorsIds = Vec::new();
            
            for i in 0..liftTasks.len() {
                println!("Лифт едет...");
                
                liftTasks.sort_by(|a: &i16, b| a.cmp(b));
                if (liftCar.floor < liftTasks[0]) {
                    liftCar.direction = Direction::Up;
                }
                liftTasks.sort_by(|a, b| b.cmp(a));
                if (liftCar.floor > liftTasks[0]) {
                    liftCar.direction = Direction::Down;
                }
            
                if matches!(liftCar.direction,Direction::Up) {
                    liftTasks.sort_by(|a: &i16, b| a.cmp(b));
                    if liftTasks[i] > liftCar.floor {
                        timeToReachFloor = ((liftTasks[i] - liftCar.floor) * 2 + 5) as u64;
                        if timeDiff >= timeToReachFloor {
                            liftReachedFloorsIds.push(i);
                            
                            timeDiff = timeDiff - timeToReachFloor;
                            liftCar.floor = liftTasks[i];
                            for f in prevFloor+1..liftCar.floor {
                                println!("Лифт на этаже{}", f);
                            }
                            prevFloor = liftCar.floor;
                            car_arrived(liftTasks[i]);
                            if (personDestination >= liftCar.floor) {
                                personFloor = liftCar.floor;
                            }
                            if personDestination == liftCar.floor {
                                println!("Лифт на Вашем этаже!");
                                //personDestination = -100;
                            }
                        } else {
                            break;
                        }
                    }
                } else {
                    liftTasks.sort_by(|a, b| b.cmp(a));
                    if liftTasks[i] < liftCar.floor {
                        timeToReachFloor = ((liftCar.floor - liftTasks[i]) * 2 + 5) as u64;;
                        if timeDiff >= timeToReachFloor {
                            liftReachedFloorsIds.push(i);
                            timeDiff = timeDiff - timeToReachFloor;
                            liftCar.floor = liftTasks[i];
                            for f in prevFloor+1..liftCar.floor {
                                println!("Лифт на этаже{}", f);
                            }
                            prevFloor = liftCar.floor;
                            car_arrived(liftTasks[i]);
                            if (personDestination <= liftCar.floor) {
                                personFloor = liftCar.floor;
                            }
                            if personDestination == liftCar.floor {
                                println!("Лифт на Вашем этаже!");
                                //personDestination = -100;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
            //Очистка этажей которые проехал лифт из очереди задач лифта
            liftReachedFloorsIds.sort_unstable_by(|a, b| b.cmp(a));
            for i in 0..liftReachedFloorsIds.len() {
                liftTasks.remove(liftReachedFloorsIds[i]);
            }
            liftReachedFloorsIds.clear();
            if personFloor == liftCar.floor {
                //println!("Лифт прибыл");
            }
        } else {
            timeDiff = 0;
        }
        
        
        println!("______________________________");
        println!("Ваш этаж {}", personFloor);
        println!("Время ожидания лифта {}", timeDiff);
        println!("Лифт на этаже {}", liftCar.floor);
        println!("Выберите действие:");
        println!("1)Вызвать лифт вверх;");
        println!("2)Вызвать лифт вниз;");
        println!("3)Выбрать и поехать на этаж;");
        println!("4)Ждать");
        println!("______________________________");
        action = input!("int");
        
        tasksStatus = liftTasks.len();
        
        match action {
            1 => {
                statusLift(personFloor, &mut liftCar, Direction::Up, &mut liftTasks);
                if liftCar.floor < personFloor {
                    liftCar.direction = Direction::Up;
                } else {
                    liftCar.direction = Direction::Down;
                }
                //break;
                }
            2 => {
                statusLift(personFloor, &mut liftCar, Direction::Down, &mut liftTasks);
                if liftCar.floor < personFloor {
                    liftCar.direction = Direction::Up;
                } else {
                    liftCar.direction = Direction::Down;
                }
                //break;
                }
            3 => {
                    println!("Выберите Ваш этаж: ");
                    personDestination = input!("int");
                    if liftCar.floor < personDestination {
                        liftCar.direction = Direction::Up;
                    } else {
                        liftCar.direction = Direction::Down;
                    }
                    liftCar.status = car_floor_button_pressed(personDestination, &mut liftTasks);
                    println!("Кто-то еще едет с Вами? Да/Нет");
                    message = input!("string");
                    while (message == "Да") {
                        println!("Выберите его(её) этаж: ");
                        liftCar.status = car_floor_button_pressed(input!("int"), &mut liftTasks);
                        println!("Кто-то еще едет с Вами? Да/Нет");
                        message = input!("string");
                    }
                    statusLift(personFloor, &mut liftCar, Direction::Down, &mut liftTasks);
                    //break;
                }
            2 => {
                println!("Вы только достали газетку, а тут.. ");
                //break;
                }
            _ => {
                    println!("Выберите число 1-4");
                    //break;
                 }
        }
        
        
    
        if personDestination == 404 {
            done = true;
        }
    }

}