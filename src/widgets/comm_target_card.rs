use crate::icons::Icon;
use gtk::prelude::*;
use relm::Widget;
use relm_derive::{widget, Msg};
use std::collections::BTreeSet;

#[derive(Msg)]
pub enum Msg {}

#[derive(Clone)]
pub struct CommTargetCardData {
    pub ip: String,
    pub port: u32,
    pub protocol_index: usize,
    pub remote_hosts: BTreeSet<String>,
    pub protocol_icon: Icon,
    pub summary_details: Option<String>,
    pub incoming_session_count: usize,
}

pub struct Model {
    data: CommTargetCardData,
}

#[widget]
impl Widget for CommTargetCard {
    fn model(relm: &relm::Relm<Self>, orig_data: CommTargetCardData) -> Model {
        let mut data = orig_data;
        if data
            .summary_details
            .as_deref()
            .filter(|details| Self::server_ip_port_display(&data).starts_with(details))
            .is_some()
        {
            // messy... meant to avoid for http to have ip+port repeated for ip+port display,
            // and then again for the details, which is the hostname, in case the hostname
            // was just the IP
            data.summary_details = None;
        }
        Model { data }
    }

    fn update(&mut self, event: Msg) {}

    fn server_ip_port_display(data: &CommTargetCardData) -> String {
        format!("{}:{}", data.ip, data.port)
    }

    fn details_display(data: &CommTargetCardData) -> String {
        format!(
            "{} remote hosts, {} sessions",
            data.remote_hosts.len(),
            data.incoming_session_count
        )
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Horizontal,
            margin_top: 7,
            margin_start: 7,
            margin_end: 7,
            margin_bottom: 7,
            gtk::Image {
                margin_end: 10,
                property_icon_name: Some(self.model.data.protocol_icon.name()),
                // https://github.com/gtk-rs/gtk/issues/837
                property_icon_size: 3, // gtk::IconSize::LargeToolbar,
            },
            gtk::Grid {
                #[style_class="target_server_ip_port"]
                gtk::Label {
                    label: &CommTargetCard::server_ip_port_display(&self.model.data),
                    cell: {
                        left_attach: 0,
                        top_attach: 1,
                    },
                },
                gtk::Label {
                    label: &CommTargetCard::details_display(&self.model.data),
                    cell: {
                        left_attach: 0,
                        top_attach: 2,
                    },
                },
                gtk::Label {
                    cell: {
                        left_attach: 0,
                        top_attach: 0,
                    },
                    label: self.model.data.summary_details.as_deref().unwrap_or(""),
                    visible: self.model.data.summary_details.is_some(),
                }
            }
        }
    }
}
