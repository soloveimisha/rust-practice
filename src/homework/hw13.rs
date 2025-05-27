#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    a: Point,
    b: Point,
}

#[derive(Debug, Clone, Copy)]
struct Event {
    x: i32,
    y1: i32,
    y2: i32,
    typ: i32, 
}

fn area_occupied(rects: &Vec<Rectangle>) -> i32 {
    let mut events = Vec::new();
    let mut ys = Vec::new();

    
    for rect in rects {
        let x1 = rect.a.x.min(rect.b.x);
        let x2 = rect.a.x.max(rect.b.x);
        let y1 = rect.a.y.min(rect.b.y);
        let y2 = rect.a.y.max(rect.b.y);

        events.push(Event { x: x1, y1, y2, typ: 1 });
        events.push(Event { x: x2, y1, y2, typ: -1 });

        ys.push(y1);
        ys.push(y2);
    }

   
    ys.sort();
    ys.dedup();

 
    let y_to_idx: std::collections::HashMap<i32, usize> = ys.iter().enumerate().map(|(i, &y)| (y, i)).collect();

  
    events.sort_by_key(|e| e.x);

    let mut count = vec![0; ys.len()];
    let mut total_area = 0;
    let mut prev_x = events[0].x;

    for event in events {
        let dx = event.x - prev_x;
        if dx > 0 {
          
            let mut covered_height = 0;
            for i in 0..ys.len() - 1 {
                if count[i] > 0 {
                    covered_height += ys[i + 1] - ys[i];
                }
            }
            total_area += covered_height * dx;
        }

        
        let i1 = y_to_idx[&event.y1];
        let i2 = y_to_idx[&event.y2];
        for i in i1..i2 {
            count[i] += event.typ;
        }

        prev_x = event.x;
    }

    total_area
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn main() {
    let data = test_data();
    let occupied = area_occupied(&data);
    println!("Occupied area: {}", occupied);
}
