use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug, Clone)]
enum WeekDay {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

#[derive(Deserialize, Debug, Clone)]
struct Activity {
    week_day: WeekDay,
    start_hour: u8,
    hour_duration: u8,
    money: f64,
    fun: f64,
}

#[derive(Deserialize, Debug, Clone)]
struct Possibilities(Vec<Activity>);

fn main() {
    let files = ["small.json", "medium.json", "large.json", "xlarge.json"];
    for file in files {
        let calendar = [[(0u8, 0usize); 24]; 7];

        println!();
        println!("Opening {}", file);
        let open_file = fs::read_to_string(format!("input/{}", file));
        let possibilities: Possibilities =
            serde_json::from_str(open_file.unwrap().as_str()).unwrap();

        let mut possibilities = Possibilities(possibilities.0.into_iter().filter(|a| a.fun > 0.0 && a.money > 0.0).collect());

        possibilities
            .0
            .sort_by(|a, b| b.fun.partial_cmp(&a.fun).unwrap());

        // Sort by most fun
        let most_fun: Possibilities = Possibilities(possibilities.0.iter().cloned().collect());

        // possibilities.0.sort_by(|a, b| b.money.partial_cmp(&a.money).unwrap());

        // Sort by least money
        // let most_profitable: Possibilities = Possibilities(possibilities.0.iter().cloned().collect());

        // println!("These are the most fun activities:");
        // for activity in most_fun.0.into_iter().take(5) {
        //     println!("{:?}", activity);
        // }

        // println!("These are the most profitable activities:");
        // for activity in most_profitable.0.into_iter().take(5) {
        //     println!("{:?}", activity);
        // }

        let mut fun_and_profitable: Possibilities = Possibilities(most_fun.0.clone().into_iter().filter(|a| a.money >= 0.).collect());
        let fun_and_not_profitable: Possibilities = Possibilities(most_fun.0.into_iter().filter(|a| a.money < 0.).collect());

        fun_and_profitable.0.extend(fun_and_not_profitable.0);
        let most_fun = fun_and_profitable;

        // A possible optimisation (in term of output and not in term of compute ):
        // Reference the activity in the calendar
        // Try to put the unused activity in the place if the fun is greater AND balance is over the cost of the activity
        let mut used_activities: Possibilities = Possibilities(vec![]);
        let mut unused_activities: Possibilities = Possibilities(vec![]);
        let mut balance = 0.;
        let mut fun = 0.;
        // Remove overlapping fun activities
        for (index, activity) in most_fun.0.into_iter().enumerate() {
            let num_day = match activity.week_day {
                WeekDay::Mon => 0,
                WeekDay::Tue => 1,
                WeekDay::Wed => 2,
                WeekDay::Thu => 3,
                WeekDay::Fri => 4,
                WeekDay::Sat => 5,
                WeekDay::Sun => 6,
            };
            let mut day_calendar = calendar[num_day];

            // First check if all are available
            let mut possible = true;
            for i in activity.start_hour..(activity.start_hour + activity.hour_duration) {
                if day_calendar[i as usize].0 == 1 {
                    // Time is already taken, skipping
                    possible = false;
                    break;
                }
            }
            if !possible {
                unused_activities.0.push(activity);
                continue;
            }

            for i in activity.start_hour..(activity.start_hour + activity.hour_duration) {
                day_calendar[i as usize] = (1, index);
            }

            balance = balance + activity.money;
            fun = fun + activity.fun;
            used_activities.0.push(activity);
        }

        // for unused in unused_activities.0.into_iter() {
        //     let num_day = match unused.week_day {
        //         WeekDay::Mon => 0,
        //         WeekDay::Tue => 1,
        //         WeekDay::Wed => 2,
        //         WeekDay::Thu => 3,
        //         WeekDay::Fri => 4,
        //         WeekDay::Sat => 5,
        //         WeekDay::Sun => 6,
        //     };
        //     let mut day_calendar = calendar[num_day];
        //
        //     for i in unused.start_hour..(unused.start_hour + unused.hour_duration) {
        //         if day_calendar[i as usize].0 == 1 {
        //             // Time is already taken, look at what is blocking
        //             let index = day_calendar[i as usize].1;
        //             let activity = used_activities.0.get(index).clone().unwrap();
        //             let mut possible = true;
        //             for j in i..(unused.start_hour + unused.hour_duration) {
        //                 if day_calendar[j as usize].1 != index {
        //                     // this activity is overlapping two others, not supported for now
        //                     possible = false;
        //                     break;
        //                 }
        //             }
        //
        //             if !possible {
        //                 continue;
        //             }
        //
        //             if unused.fun > activity.fun && (unused.money >= 0.0 || (unused.money.abs() < balance )) {
        //                 // unused is more preferrable
        //
        //                 // remove hours from current activity
        //                 for j in activity.start_hour..(activity.start_hour + activity.hour_duration) {
        //                     day_calendar[j as usize] = (0, 0);
        //                 }
        //
        //                 // add the unused activity
        //                 for j in unused.start_hour..(unused.start_hour + unused.hour_duration) {
        //                     day_calendar[j as usize] = (1, index);
        //                 }
        //
        //                 // Replace the activity in the list in-place
        //                 balance = balance - activity.money;
        //                 fun = fun - activity.fun;
        //                 balance = balance + unused.money;
        //                 fun = fun + unused.fun;
        //                 used_activities.0.insert(index, unused);
        //             }
        //
        //             break;
        //         }
        //     }
        // }


        // used_activities.0.sort_by(|a, b| b.money.partial_cmp(&a.money).unwrap());

        println!("Balance: {}", balance);
        println!("Fun: {}", fun);

        // while balance <= 0.0 {
        //     let popped_elem = used_activities.0.pop();
        //
        //     if popped_elem.is_none() {
        //         break;
        //     }
        //     let elem = popped_elem.unwrap();
        //     balance = balance - elem.money;
        //     fun = fun - elem.fun;
        // }

        used_activities.0.sort_by(|a, b| b.fun.partial_cmp(&a.fun).unwrap());

        while fun <= 0.0 {
            let popped_elem = used_activities.0.pop();

            if popped_elem.is_none() {
                break;
            }
            let elem = popped_elem.unwrap();
            balance = balance - elem.money;
            fun = fun - elem.fun;
        }

        let mut index = 0;
        let len = used_activities.0.len();
        while balance >= 0.0 && index < len {
            let popped_elem = used_activities.0.pop();

            if popped_elem.is_none() {
                break;
            }
            let elem = popped_elem.unwrap();
            if (elem.fun >= 0.0 && balance >= 0.0) || (elem.money <= 0.0 && elem.money.abs() > balance ) {
                // keep fun activities
                // keep the activities that pays
                used_activities.0.push(elem);
                index += 1;
                continue;
            }else if elem.money <= 0.0 && elem.money.abs() <= balance {
                balance = balance - elem.money;
                fun = fun - elem.fun;
                index += 1;
            }

            // Dont remove net positives, only remove net negatives
            if elem.fun <= 0.0 || (balance <= 0.0 && elem.money >= 0.0) {

            }else {
                used_activities.0.push(elem);
            }
        }

        println!("These are the possible activities (5 first):");
        for activity in used_activities.0.into_iter().take(20) {
            println!("{:?}", activity);
        }

        println!("Balance: {}", balance);
        println!("Fun: {}", fun);
    }
}
