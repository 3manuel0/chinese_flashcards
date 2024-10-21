#![windows_subsystem = "windows"]
use rand::Rng;
use serde_json::json;
use serde_json::Value;
use slint::CloseRequestResponse;
use slint::ComponentHandle;
use slint::SharedString;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use std::rc::Rc;
slint::include_modules!();

fn write(file: &mut File, v: HashMap<String, Value>) {
    file.seek(SeekFrom::Start(0)).unwrap();
    // file.write_all(&serialized.as_bytes()).expect("failed");
    serde_json::to_writer(file, &v).expect("file should open read only");
}

fn main() {
    let app = MainWindow::new().unwrap();
    let file = Rc::new(RefCell::new(
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("cards.json")
            .expect("failed"),
    ));
    let v: Rc<RefCell<HashMap<String, Value>>> = {
        let file_ref = file.borrow_mut(); // Mutably borrow the file
        Rc::new(RefCell::new(
            serde_json::from_reader(&*file_ref).unwrap_or_default(), // Read from the file and deserialize JSON
        ))
    };
    let len = v.borrow().keys().len();
    let mut num = rand::thread_rng().gen_range(0..len);
    let char_simpl = &v
        .borrow()
        .values()
        .nth(num)
        .and_then(Value::as_array)
        .unwrap()
        .get(0)
        .and_then(Value::as_str)
        .map(|s| s.to_string())
        .unwrap();
    let penyin = &v
        .borrow()
        .values()
        .nth(num)
        .and_then(Value::as_array)
        .unwrap()
        .get(1)
        .and_then(Value::as_str)
        .map(|s| s.to_string())
        .unwrap();
    let char_trad = &v
        .borrow()
        .values()
        .nth(num)
        .and_then(Value::as_array)
        .unwrap()
        .get(2)
        .and_then(Value::as_str)
        .map(|s| s.to_string())
        .unwrap();
    // let text = SharedString::from(str);
    app.set_char_simpl(SharedString::from(char_simpl.to_string()));
    app.set_char_trad(SharedString::from(char_trad.to_string()));
    app.set_penyen(SharedString::from(penyin.to_string()));
    app.set_place_holder(SharedString::from("show penying"));
    // let serialized = serde_json::to_string(&v).unwrap();
    // let mut writer = BufWriter::new(file);
    // app.global::<Next>().on_button_pressed(){}
    if 5 == 2 {
        v.borrow_mut().insert(
            String::from((v.borrow().keys().len()).to_string()),
            json!(["朋友", "péng you", "朋友"]),
        );
    }
    app.on_add({
        let app = app.as_weak().unwrap();
        let map = Rc::clone(&v);
        move || {
            let inp_char_simpl = app.get_inp_char_simpl();
            let inp_char_trad = app.get_inp_char_trad();
            let inp_penyen = app.get_inp_penyen();
            let new_key = map.borrow().keys().len().to_string();
            map.borrow_mut().insert(
                new_key,
                json!([*inp_char_simpl, *inp_penyen, *inp_char_trad,]),
            );
            app.set_inp_char_simpl(SharedString::from("Simplified"));
            app.set_inp_char_trad(SharedString::from("Traditional"));
            app.set_inp_penyen(SharedString::from("Pinyin"));
        }
    });
    app.on_next({
        let app = app.as_weak().unwrap();
        let v_ref = Rc::clone(&v);
        move || {
            let len = v_ref.borrow().keys().len();
            let mut newnum = rand::thread_rng().gen_range(0..len);
            while num == newnum {
                newnum = rand::thread_rng().gen_range(0..len);
            }
            let char_simpl = &v_ref
                .borrow()
                .values()
                .nth(newnum)
                .and_then(Value::as_array)
                .unwrap()
                .get(0)
                .and_then(Value::as_str)
                .map(|s| s.to_string())
                .unwrap();
            let penyin = &v_ref
                .borrow()
                .values()
                .nth(newnum)
                .and_then(Value::as_array)
                .unwrap()
                .get(1)
                .and_then(Value::as_str)
                .map(|s| s.to_string())
                .unwrap();
            let char_trad = &v_ref
                .borrow()
                .values()
                .nth(newnum)
                .and_then(Value::as_array)
                .unwrap()
                .get(2)
                .and_then(Value::as_str)
                .map(|s| s.to_string())
                .unwrap();

            app.set_char_simpl(SharedString::from(char_simpl.to_string()));
            app.set_char_trad(SharedString::from(char_trad.to_string()));
            app.set_penyen(SharedString::from(penyin.to_string()));
            app.set_place_holder(SharedString::from("show penying"));
            num = newnum;
        }
    });
    app.window().on_close_requested({
        // let app = app.as_weak().unwrap();
        let map = Rc::clone(&v);
        let file = file;
        move || -> CloseRequestResponse {
            let mut file_ref = file.borrow_mut();
            write(&mut *file_ref, map.borrow().clone());
            CloseRequestResponse::HideWindow
        }
    });
    // writer.flush().expect("file should open read only");
    // file.flush().expect("file should open read only");
    app.run().unwrap();
}
