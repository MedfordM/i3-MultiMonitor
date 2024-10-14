extern crate i3ipc;

use std::{process::Command, collections::HashMap};

use i3ipc::{I3Connection, I3EventListener, Subscription, event::{Event, BindingEventInfo}};

fn main() {

    let mut connection = I3Connection::connect().unwrap();
    println!("Established connection to i3 (version {})", connection.get_version().unwrap().major);
    let mut listener = I3EventListener::connect().unwrap();
    println!("Connected to event listener");

    listener.subscribe(&[Subscription::Binding]).unwrap();

    for event in listener.listen() {
        match event.unwrap() {
            Event::ModeEvent(_) => todo!(),
            Event::WorkspaceEvent(_) => todo!(),
            Event::OutputEvent(_) => todo!(),
            Event::WindowEvent(_) => todo!(),
            Event::BarConfigEvent(_) => todo!(),
            Event::BindingEvent(e) => if e.binding.command == "nop" {parse_workspace_monitor(&mut connection,  &e)},
            Event::ShutdownEvent(_) => todo!(),
        }
    }
}

fn update_eww_workspaces(workspace: i32, monitor: String) {
    let mut workspace_text = String::from("");
    workspace_text.push('"');
    for i in 1..6 {
        if i32::from(i) == workspace {
            workspace_text.push('󰄯');
        } else {
            workspace_text.push('󰄰');
        }
        if i != 5 {
            workspace_text.push_str("  ");
        }
    }
    workspace_text.push('"');
    let mut command = String::from("eww update workspaces_");
    command.push_str(&monitor);
    command.push('=');
    command.push_str(workspace_text.as_str());
    println!("Executing eww command: {}", command);
    Command::new("sh").arg("-c").arg(command).output().ok();
}

fn parse_workspace_monitor(connection: &mut I3Connection, binding: &BindingEventInfo) {
    let monitor_indexes: HashMap<String, i32> = HashMap::from([
        ("DP-0".to_string(), 1),
        ("DP-2".to_string(), 2),
        ("DP-4".to_string(), 3)
    ]);
    
    let workspace_keys: HashMap<String, i32> = HashMap::from([
        ("a".to_string(), 1),
        ("r".to_string(), 2),
        ("s".to_string(), 3),
        ("t".to_string(), 4),
        ("d".to_string(), 5)
    ]);

    let workspaces = connection.get_workspaces().unwrap().workspaces;
    let active_monitor = &workspaces.iter().filter(|workspace| workspace.focused == true).next().unwrap().output;
    let active_monitor_index = monitor_indexes.get(active_monitor).unwrap();
    let key:String = binding.binding.symbol.as_ref().unwrap().to_string();
    let requested_space_index = workspace_keys.get(&key).unwrap().to_owned();
    let requested_space = requested_space_index + ((active_monitor_index - 1) * 5);
    // println!("Requested workspace was {:?} (index {:?}) on monitor {:?} (index {:?})", &requested_space, &requested_space_index, &active_monitor, &active_monitor_index);

    let mut command:String;

    let mods:&Vec<String> = &binding.binding.event_state_mask;
    if mods.contains(&String::from("shift")) {
        command = String::from("move container to workspace ");
    } else {
        command = String::from("workspace ");
    }

    command.push_str(&requested_space.to_string());

    println!("Executing i3 command: {}", command);
    let result = connection.run_command(command.as_str());

    if result.is_ok() && !mods.contains(&String::from("shift")) {
        update_eww_workspaces(requested_space_index, active_monitor_index.to_string());
    }
}
