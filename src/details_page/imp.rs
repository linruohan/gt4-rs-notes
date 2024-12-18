use std::cell::RefCell;

use glib::Binding;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, TextView, Button};

use adw::NavigationPage;
use adw::subclass::prelude::NavigationPageImpl;

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/github/linruohan/notes/ui/details_page.ui")]
pub struct DetailsPage {
    #[template_child]
    pub edit_button: TemplateChild<Button>,

    #[template_child]
    pub save_button: TemplateChild<Button>,

    #[template_child]
    pub cancel_button: TemplateChild<Button>,

    #[template_child]
    pub delete_button: TemplateChild<Button>,

    #[template_child]
    pub text_view: TemplateChild<TextView>,

    pub bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for DetailsPage {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "DetailsPage";
    type Type = super::DetailsPage;
    type ParentType = NavigationPage;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for DetailsPage {}

// Trait shared by all widgets
impl WidgetImpl for DetailsPage {}

// Trait shared by all boxes
impl NavigationPageImpl for DetailsPage {}