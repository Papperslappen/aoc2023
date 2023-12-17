use std::collections::{HashMap, hash_map::Entry};
use pom::{parser::*, char_class::alpha};
use util::parser::posint;

fn silly_hash(s:&String)->u32{
    s.as_bytes().iter().fold(0_u32,|current_value,c|{
        ((current_value + *c as u32)*17)%256
    })
}

fn solution_a(input: &[String]) -> u32 {
    input.iter().map(silly_hash).sum()
}

enum Op{
    Eq(String,u32),
    Minus(String),
}

fn label<'a>()->Parser<'a,u8,String>{
    is_a(alpha).repeat(1..).collect().map(|u|String::from_utf8(Vec::from(u)).unwrap())
}

fn solution_b(input: &[String]) -> u32 {
    let mut boxes = HashMap::<u32,Vec<(String, u32)>>::new();
    let parser = (label()+sym(b'=')*posint()).map(|(l,n)| Op::Eq(l,n)) | (label()-sym(b'-')).map(Op::Minus);
    input.iter().map(|s| parser.parse(s.as_bytes()).unwrap()).for_each(|op|
    {
        match op {
            Op::Eq(label, number) => {
                let box_no =silly_hash(&label);
                match boxes.entry(box_no){
                    std::collections::hash_map::Entry::Occupied(mut v) => {
                        let contents = v.get_mut();
                        if let Some(p) = contents.iter().position(|(l,_)| *l==label){
                            contents[p] = (label,number)
                        } else {
                            contents.push((label,number))
                        }
                    },
                    std::collections::hash_map::Entry::Vacant(v) => {v.insert(vec![(label,number)]);},
                };
                },
            Op::Minus(label) => {
                let box_no = silly_hash(&label);
                if let Entry::Occupied(mut v) = boxes.entry(box_no) {
                    let contents = v.get_mut(); 
                    if let Some(p) = contents.iter().position(|(l,_)| *l == label){
                        contents.remove(p);
                    }
                }
            },
        }
    });

    (0..256).filter_map(|box_no|{
        Some(boxes.get(&box_no)?
            .iter()
            .enumerate()
            .map(|(slot,(_,n))| *n * (box_no + 1) * (slot as u32 +1)).sum::<u32>())
        }).sum()
}

#[test]
fn test_solutions() {
    use util::raw_to_strings;
    let input1 = raw_to_strings(
        "HASH",
    );
    let input2 = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string().split(',').map(String::from).collect::<Vec<_>>();
    assert_eq!(solution_a(&input1), 52);
    assert_eq!(solution_a(input2.as_slice()), 1320);
    assert_eq!(solution_b(input2.as_slice()), 145);
}

fn main() {
    println!("input:");
    let input = util::get_input_rows().first().unwrap().split(',').map(String::from).collect::<Vec<_>>();
    println!("Answer puzzle A: {}", solution_a(&input));
    println!("Answer puzzle B: {}", solution_b(&input));
}
