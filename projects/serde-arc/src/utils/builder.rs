use crate::indent;

pub fn build_dict(kv: Vec<String>) -> String {
    match kv.len() {
        0 => return String::from("{}"),
        1 => {
            let t = &kv[0];
            if t.lines().count() == 1 {
                return format!("{{{}}}", t);
            }
        }
        _ => (),
    }
    return format!("{{\n{}}}", indent(&kv.join("\n"), "    "));
}

pub fn build_list(ts: Vec<String>) -> String {
    let mut max = 0;
    let mut len = 0;
    let mut v = vec![];
    for s in ts {
        let l = s.lines().count();
        if l > max {
            max = l
        }
        len += s.len(); //for faster
        // len += s.chars().count();
        v.push(s)
    }
    if len < 128 && max <= 1 { format!("[{}]", v.join(", ")) } else { format!("[\n{}]", indent(&v.join("\n"), "    ")) }
}
