use flowtable::{FlowTable, FlowTableValue};

mod statematrix;
mod flowtable;

fn main() {
    let mut flowtable = FlowTable::new(5, 4);

    flowtable.set_entry(0, 0, FlowTableValue::Value(1), FlowTableValue::Value(0));
    flowtable.set_entry(0, 1, FlowTableValue::DontCare, FlowTableValue::Value(1));
    flowtable.set_entry(0, 2, FlowTableValue::Value(2), FlowTableValue::DontCare);
    flowtable.set_entry(0, 3, FlowTableValue::Value(1), FlowTableValue::Value(0));
    
    flowtable.set_entry(1, 0, FlowTableValue::Value(2), FlowTableValue::Value(0));
    flowtable.set_entry(1, 1, FlowTableValue::Value(4), FlowTableValue::Value(1));
    flowtable.set_entry(1, 2, FlowTableValue::Value(1), FlowTableValue::Value(0));
    flowtable.set_entry(1, 3, FlowTableValue::DontCare, FlowTableValue::DontCare);

    flowtable.set_entry(2, 0, FlowTableValue::Value(2), FlowTableValue::Value(0));
    flowtable.set_entry(2, 1, FlowTableValue::Value(3), FlowTableValue::Value(1));
    flowtable.set_entry(2, 2, FlowTableValue::DontCare, FlowTableValue::DontCare);
    flowtable.set_entry(2, 3, FlowTableValue::Value(4), FlowTableValue::Value(0));

    flowtable.set_entry(3, 0, FlowTableValue::DontCare, FlowTableValue::DontCare);
    flowtable.set_entry(3, 1, FlowTableValue::Value(0), FlowTableValue::Value(1));
    flowtable.set_entry(3, 2, FlowTableValue::Value(1), FlowTableValue::DontCare);
    flowtable.set_entry(3, 3, FlowTableValue::DontCare, FlowTableValue::DontCare);

    flowtable.set_entry(4, 0, FlowTableValue::DontCare, FlowTableValue::DontCare);
    flowtable.set_entry(4, 1, FlowTableValue::DontCare, FlowTableValue::DontCare);
    flowtable.set_entry(4, 2, FlowTableValue::Value(1), FlowTableValue::Value(1));
    flowtable.set_entry(4, 3, FlowTableValue::DontCare, FlowTableValue::DontCare);

    println!("original:");
    println!("{flowtable}");
    flowtable = flowtable.reduce();

    println!("reduced:");
    println!("{flowtable}");
}
