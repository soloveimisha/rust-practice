fn main() {
    let solutions = solve();
    println!("Найдено решений: {}", solutions.len());
    for (m,u,x,a,s,l,o,n) in solutions {
        println!("  {}{}{}{}", m,u,x,a);
        println!("×     {}", a);
        println!("--------");
        println!("  {}{}{}{}", s,l,o,n);
        println!();
    }
}

fn solve() -> Vec<(u8,u8,u8,u8,u8,u8,u8,u8)> {
    let mut results = Vec::new();

    for m in 1..=8 {
        for u in 1..=8 {
            if u == m { continue; }
            for x in 1..=8 {
                if x == m || x == u { continue; }
                for a in 1..=8 {
                    if a == m || a == u || a == x { continue; }
                    for s in 1..=8 {
                        if s == m || s == u || s == x || s == a { continue; }
                        for l in 1..=8 {
                            if l == m || l == u || l == x || l == a || l == s { continue; }
                            for o in 1..=8 {
                                if o == m || o == u || o == x || o == a || o == s || o == l { continue; }
                                for n in 1..=8 {
                                    if n == m || n == u || n == x || n == a || n == s || n == l || n == o { continue; }

                                    let muxa = m as u32 * 1000 + u as u32 * 100 + x as u32 * 10 + a as u32;
                                    let slon = s as u32 * 1000 + l as u32 * 100 + o as u32 * 10 + n as u32;

                                    if muxa * a as u32 == slon {
                                        results.push((m,u,x,a,s,l,o,n));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    results
}
