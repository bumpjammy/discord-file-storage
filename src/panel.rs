use rocket::{FromForm, get, post, State};
use rocket::form::Form;
use rocket::fs::TempFile;
use crate::message::{Message, MessageChannel, MessageType};

#[derive(FromForm)]
struct Upload<'f> {
    file: TempFile<'f>
}

#[post("/upload", format = "multipart/form-data", data = "<form>")]
pub async fn upload_file(
    channel: &State<MessageChannel>,
    mut form: Form<Upload<'_>>
) -> std::io::Result<()> {
    let dir = "./temp/".to_string();
    let file_name = &form.file.name().expect("Failed to get file name").to_string();
    let file_extension = &form.file
        .content_type().expect("Failed to get file extension")
        .extension().expect("Failed to get file extension")
        .to_string();
    let full_file_name = file_name.to_owned() + "." + file_extension;
    let file_path = dir + &full_file_name;
    form.file.persist_to(&file_path).await?;
    channel.1.send(
        Message(
            MessageType::UploadFile,
            Some(file_path + ":" + &full_file_name)
        )
    )
    .await
    .expect("Failed to upload file");
    Ok(())
}

#[get("/get_files")]
pub async fn get_files(channel: &State<MessageChannel>) -> String {
    channel.1.send(
        Message(
            MessageType::GetFiles,
            None
        )
    )
    .await
    .expect("Failed to send message");

    while let Some(msg) = channel.0.lock().await.recv().await {
        match msg.0 {
            MessageType::GetFiles => {
                return msg.1.expect("No files returned");
            }
            _ => {}
        }
    }

    return "".to_string();
}