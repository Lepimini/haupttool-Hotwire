use super::http_body_widget::HttpBodyWidget;
use super::http_message_parser::HttpMessageData;
use crate::icons::Icon;
use crate::message_parser::MessageInfo;
use crate::widgets::comm_info_header;
use crate::widgets::comm_info_header::CommInfoHeader;
use crate::widgets::comm_remote_server::MessageData;
use crate::BgFunc;
use gtk::prelude::*;
use relm::Widget;
use relm_derive::{widget, Msg};
use std::path::PathBuf;
use std::sync::mpsc;

#[derive(Msg, Debug)]
pub enum Msg {
    DisplayDetails(mpsc::Sender<BgFunc>, PathBuf, MessageInfo),
    RemoveFormatToggled,
}

pub struct Model {
    stream_id: u32,
    client_ip: String,
    data: HttpMessageData,

    format_request_response: bool,
}

#[widget]
impl Widget for HttpCommEntry {
    fn model(
        relm: &relm::Relm<Self>,
        params: (u32, String, HttpMessageData, gtk::Overlay),
    ) -> Model {
        let (stream_id, client_ip, data, overlay) = params;

        let disable_formatting_btn = gtk::ToggleButtonBuilder::new()
            .label("Disable formatting")
            .always_show_image(true)
            .image(&gtk::Image::from_icon_name(
                Some(Icon::REMOVE_FORMAT.name()),
                gtk::IconSize::Menu,
            ))
            .valign(gtk::Align::Start)
            .halign(gtk::Align::End)
            .margin_top(10)
            .margin_end(10)
            .build();
        overlay.add_overlay(&disable_formatting_btn);
        relm::connect!(
            relm,
            disable_formatting_btn,
            connect_clicked(_),
            Msg::RemoveFormatToggled
        );
        Model {
            data,
            stream_id,
            client_ip,
            format_request_response: true,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::DisplayDetails(
                bg_sender,
                file_path,
                MessageInfo {
                    client_ip,
                    stream_id,
                    message_data: MessageData::Http(msg),
                },
            ) => {
                self.model.data = msg;
                self.streams
                    .comm_info_header
                    .emit(comm_info_header::Msg::Update(client_ip.clone(), stream_id));
                self.model.stream_id = stream_id;
                self.model.client_ip = client_ip;
            }
            Msg::RemoveFormatToggled => {
                self.model.format_request_response = !self.model.format_request_response;
            }
            _ => {}
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            margin_top: 10,
            margin_bottom: 10,
            margin_start: 10,
            margin_end: 10,
            spacing: 10,
            #[name="comm_info_header"]
            CommInfoHeader(self.model.client_ip.clone(), self.model.stream_id) {
            },
            #[style_class="http_first_line"]
            gtk::Label {
                label: &self.model.data.request.as_ref().map(|r| r.first_line.as_str()).unwrap_or("Missing request info"),
                xalign: 0.0,
                selectable: true,
            },
            gtk::Label {
                label: &self.model.data.request.as_ref().map(|r| r.other_lines.as_str()).unwrap_or(""),
                xalign: 0.0,
                selectable: true,
            },
            HttpBodyWidget(),
            gtk::Separator {},
            #[style_class="http_first_line"]
            gtk::Label {
                label: &self.model.data.response.as_ref().map(|r| r.first_line.as_str()).unwrap_or("Missing response info"),
                xalign: 0.0,
                selectable: true,
            },
            gtk::Label {
                label: &self.model.data.response.as_ref().map(|r| r.other_lines.as_str()).unwrap_or(""),
                xalign: 0.0,
                selectable: true,
            },
            HttpBodyWidget(),
        }
    }
}