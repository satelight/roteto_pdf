use lopdf::Document;

pub async fn rotate_one_page_pdf(pdf_path:&str,angle:i64,changing_page_number:u32){
    let mut doc = Document::load(pdf_path).unwrap();
    for (page_number, page_id) in doc.get_pages() {
        if page_number == changing_page_number {
            let page_dict = doc.get_object_mut(page_id)
                .and_then(|obj| obj.as_dict_mut())
                .expect("Missing page!");
    
            let current_rotation = page_dict
            .get(b"Rotate")
            .and_then(|obj| obj.as_i64())
            .unwrap_or(0);
    
            // Add the angle and update
            page_dict.set("Rotate", (current_rotation + angle) % 360);
        }
    }
    
    doc.save(pdf_path).unwrap();
}

pub async fn rotate_all_pdf(pdf_path:&str,angle:i64){
    let mut doc = Document::load(pdf_path).unwrap();
    for (_page_number, page_id) in doc.get_pages() {
        let page_dict = doc.get_object_mut(page_id)
            .and_then(|obj| obj.as_dict_mut())
            .expect("Missing page!");

        let current_rotation = page_dict
        .get(b"Rotate")
        .and_then(|obj| obj.as_i64())
        .unwrap_or(0);

        // Add the angle and update
        page_dict.set("Rotate", (current_rotation + angle) % 360);
        
    }
    
    doc.save(pdf_path).unwrap();

}

#[tokio::main]
#[test]
async fn it_works() {
    rotate_all_pdf("sample_pdf/dummy2.pdf",90).await;
}
