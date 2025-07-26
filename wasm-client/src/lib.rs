mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlButtonElement, Response}; // 只保留实际使用的类型
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(_name: &str) { // 标记未使用的参数
    alert("Hello, wasm-client!"); // 移除不必要的unsafe块
}

pub mod error;
pub mod models;
use crate::models::course::{delete_course, get_courses_by_teacher, Course};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("no global document exists");

    let left_body = document
        .get_element_by_id("left-tbody")
        .expect("left div not exists");

    let courses: Vec<Course> = get_courses_by_teacher(1).await.unwrap();

    for c in courses.iter() {
        let tr = document.create_element("tr")?;
        tr.set_attribute("id", format!("tr-{}", c.id).as_str())?;
        
        let td = document.create_element("td")?;
        td.set_text_content(Some(format!("{}", c.id).as_str()));
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        td.set_text_content(c.name.as_deref());
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        if let Some(time) = &c.time {
            td.set_text_content(Some(&time.format("%Y-%m-%d").to_string()));
        }
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        if let Some(desc) = &c.description { // 直接借用，避免克隆
            td.set_text_content(Some(desc));
        }
        tr.append_child(&td)?;

        let td = document.create_element("td")?;

        let btn: HtmlButtonElement = document
            .create_element("button")?
            .dyn_into::<HtmlButtonElement>()?;
         
        let cid = c.id;
        let window_clone = window.clone(); // 克隆window供闭包使用
        
        let click_closure = Closure::wrap(
        Box::new(move |_event: web_sys::MouseEvent| 
        {
            let r = window_clone.confirm_with_message("确定删除?").unwrap();
            if r 
            {
                let window_for_async = window_clone.clone(); // 再次克隆供异步块使用
                spawn_local(async move 
                    {
                    if let Err(e) = delete_course(1, cid).await {
                        log(&format!("删除失败: {:?}", e));
                    } else {
                        alert("删除成功");
                        window_for_async.location().reload().unwrap(); // 使用新克隆的window
                    }
                });
            }
        }) as Box<dyn FnMut(web_sys::MouseEvent)>
);

        btn.add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())?;
        click_closure.forget();

        btn.set_attribute("class", "btn btn-danger btn-sm")?;
        btn.set_text_content(Some("Delete"));
        td.append_child(&btn)?;
        tr.append_child(&td)?;

        left_body.append_child(&tr)?;
    }
    Ok(())
}