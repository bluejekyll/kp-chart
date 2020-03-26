use log::{debug, error};
use serde::{Deserialize, Serialize};
use web_sys::HtmlSelectElement;
use yew::callback::Callback;
use yew::format::Json;
use yew::prelude::*;
use yew::services::{storage::Area, StorageService};

use crate::data::*;

const PEOPLE_KEY: &str = "people_v1";
type IsEditting = bool;
type Id = usize;

pub enum PeopleMsg {
    AddPerson,
    SavePeople,
    EditPerson(Id),
    DeletePerson(Id),
    PersonNameInput(Id, String),
    PersonAbilityInput(Id, Ability),
}

#[derive(Clone)]
pub struct PeopleModel {
    inc: usize,
    people: Vec<(Person, IsEditting)>,
    on_save: Option<Callback<usize>>,
    link: ComponentLink<Self>,
}

#[derive(Clone, Default, PartialEq, Properties)]
pub struct PeopleProps {
    pub on_save: Option<Callback<usize>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PeopleStore {
    pub inc: usize,
    pub people: Vec<Person>,
}

impl PeopleStore {
    pub fn restore(local_store: &mut StorageService) -> Option<Self> {
        let from_store = local_store.restore(PEOPLE_KEY);
        match from_store {
            Json(Ok(people)) => Some(people),
            // TODO: reset local store...
            Json(Err(err)) => {
                error!("could not load from local store: {}", err);
                None
            }
        }
    }

    pub fn store(&mut self, local_store: &mut StorageService) {
        self.inc += 1;
        debug!("saving people: {}", self.inc);
        local_store.store(PEOPLE_KEY, Json(self as &Self));
    }
}

impl From<PeopleModel> for PeopleStore {
    fn from(model: PeopleModel) -> Self {
        Self {
            inc: model.inc,
            people: model.people.into_iter().map(|(p, _)| p).collect(),
        }
    }
}

impl PeopleModel {
    fn from(
        model: PeopleStore,
        on_save: Option<Callback<usize>>,
        link: ComponentLink<Self>,
    ) -> Self {
        Self {
            inc: model.inc,
            people: model.people.into_iter().map(|p| (p, false)).collect(),
            on_save,
            link,
        }
    }
}

impl Component for PeopleModel {
    type Message = PeopleMsg;
    type Properties = PeopleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        debug!("creating PeopleModel");

        let mut local_store = StorageService::new(Area::Local).expect("failed to get storage");

        match PeopleStore::restore(&mut local_store) {
            Some(this) => Self {
                inc: this.inc,
                people: this.people.into_iter().map(|p| (p, false)).collect(),
                on_save: props.on_save,
                link,
            },
            None => {
                let people = crate::default_people();
                // TODO: make a borrowed type
                let mut people = PeopleStore {
                    inc: 0,
                    people: people,
                };

                people.store(&mut local_store);
                Self {
                    inc: people.inc,
                    people: people.people.into_iter().map(|p| (p, false)).collect(),
                    on_save: props.on_save,
                    link,
                }
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PeopleMsg::SavePeople => {
                debug!("saving PeopleModel");
                let mut local_store =
                    StorageService::new(Area::Local).expect("failed to get storage");
                let mut people: PeopleStore = self.clone().into();
                people.store(&mut local_store);
                *self = PeopleModel::from(people, self.on_save.take(), self.link.clone());

                self.on_save.as_ref().map(|e| e.emit(self.inc));
                true
            }
            PeopleMsg::AddPerson => {
                debug!("adding a Person");
                let person = Person::new("Jane Doe", Ability::Adult);
                self.people.push((person, true));
                true
            }
            PeopleMsg::EditPerson(id) => {
                debug!("edit person: {}", id);
                self.people
                    .get_mut(id)
                    .map(|p| {
                        if !p.1 {
                            p.1 = true;
                            true
                        } else {
                            false
                        }
                    })
                    .unwrap_or(false)
            }
            PeopleMsg::DeletePerson(idx) => {
                let person = self.people.remove(idx);
                debug!("deleted {:?}", person);
                true
            }
            PeopleMsg::PersonNameInput(id, name) => self
                .people
                .get_mut(id)
                .map(|p| {
                    debug!("saving name: {}", name);
                    if p.0.name() != name {
                        p.0.set_name(name);
                        true
                    } else {
                        false
                    }
                })
                .unwrap_or(false),
            PeopleMsg::PersonAbilityInput(id, ability) => self
                .people
                .get_mut(id)
                .map(|p| {
                    debug!("saving name: {}", ability);
                    if p.0.ability() != ability {
                        p.0.set_ability(ability);
                        true
                    } else {
                        false
                    }
                })
                .unwrap_or(false),
        }
    }

    fn view(&self) -> Html {
        // let select = |is_selected: bool| {
        //     html!{
        //         <Select: is_selected={is_selected}, />
        //     }
        // };

        let edit_delete = |id: Id, is_editting: IsEditting, link: &ComponentLink<Self>| {
            let on_edit = link.callback(PeopleMsg::EditPerson);
            let on_delete = link.callback(PeopleMsg::DeletePerson);

            html! {
                <EditDelete: id={id}, is_editting={is_editting}, on_edit=on_edit, on_delete=on_delete, />
            }
        };
        let person_row = |id: Id, person: &(Person, IsEditting), link: &ComponentLink<Self>| {
            let name_on_input = link.callback(|(i, n)| PeopleMsg::PersonNameInput(i, n));
            let ability_on_input = link.callback(|(i, a)| PeopleMsg::PersonAbilityInput(i, a));

            html! {
                <tr>
                    <td><PersonName: id={id}, name={person.0.name().clone()}, is_editting={person.1}, on_input=name_on_input,/></td>
                    <td><PersonAbility: id={id}, ability={person.0.ability()}, is_editting={person.1}, on_input=ability_on_input,/></td>
                    <td class="edit_delete",>{ edit_delete(id, person.1, &self.link) }</td>
                </tr>
            }
        };

        html! {
            <>
                <h2>{"All the beautiful people"}</h2>
                <table>
                    <thead>
                        <tr><th>{"Person"}</th><th>{"Ability"}</th><th>{" "}</th></tr>
                    </thead>
                    <tbody>
                        { for self.people.iter().enumerate().map(|(i, p)| person_row(i, p, &self.link)) }
                    </tbody>
                    <tfoot>
                        <tr><td>
                            <button onclick=self.link.callback(|_| PeopleMsg::AddPerson), >
                                <i class=("fa", "fa-plus-square"), aria-hidden="true",></i>
                            </button>
                            <button onclick=self.link.callback(|_| PeopleMsg::SavePeople), >
                                <i class=("fa", "fa-floppy-o"), aria-hidden="true",></i>
                            </button>
                            //button onclick=|_| PeopleMsg::SavePeople, >{"Save all the People"}</button>
                        </td></tr>
                    </tfoot>
                </table>
            </>
        }
    }
}

// #[derive(Clone, Eq, PartialEq, Default)]
// struct Select {
//     is_selected: bool,
// }

// impl Component<Context> for Select {
//     type Message = ();
//     type Properties = Self;

//     fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {
//         Self {
//             is_selected: props.is_selected,
//         }
//     }

//     fn update(&mut self, _msg: Self::Message, _context: &mut Env<Context, Self>) -> ShouldRender {
//         true
//     }

//     fn change(
//         &mut self,
//         _props: Self::Properties,
//         _context: &mut Env<Context, Self>,
//     ) -> ShouldRender {
//         true
//     }
// }

// impl Renderable<Context, Select> for Select {
//     fn view(&self) -> Html<Context, Self> {
//         html! {
//             <input type="checkbox", checked=true,/>
//         }
//     }
// }

/// EditDelete Component for a person row
#[derive(Clone)]
struct EditDelete {
    id: Id,
    is_editting: IsEditting,
    on_edit: Option<Callback<Id>>,
    on_delete: Option<Callback<Id>>,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Default, Properties)]
struct EditDeleteProps {
    pub id: Id,
    pub is_editting: IsEditting,
    pub on_edit: Option<Callback<Id>>,
    pub on_delete: Option<Callback<Id>>,
}

enum EditDeleteMsg {
    Edit,
    Delete,
}

impl Component for EditDelete {
    type Message = EditDeleteMsg;
    type Properties = EditDeleteProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            id: props.id,
            is_editting: props.is_editting,
            on_edit: props.on_edit,
            on_delete: props.on_delete,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            EditDeleteMsg::Edit => {
                debug!("editting: {}", self.id);
                if !self.is_editting {
                    self.on_edit.as_ref().map(|c| c.emit(self.id));
                }
            }
            EditDeleteMsg::Delete => {
                debug!("deleting: {}", self.id);
                self.on_delete.as_ref().map(|c| c.emit(self.id));
            }
        }

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.is_editting != props.is_editting {
            self.is_editting = props.is_editting;
            return true;
        }
        false
    }

    fn view(&self) -> Html {
        let disabled = if self.is_editting { "disabled" } else { "" };

        html! {
            <div class="edit_delete", >
                <i class=("fa", "fa-pencil-square-o", "fa-fw", disabled), aria-hidden="true", onclick=self.link.callback(|_| EditDeleteMsg::Edit), />
                <i class=("fa", "fa-trash", "fa-fw"), aria-hidden="true", onclick=self.link.callback(|_| EditDeleteMsg::Delete), />
            </div>
        }
    }
}

#[derive(Clone)]
struct PersonName {
    id: Id,
    name: String,
    is_editting: IsEditting,
    on_input: Option<Callback<(Id, String)>>,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Default, Properties)]
struct PersonNameProps {
    pub id: Id,
    pub name: String,
    pub is_editting: IsEditting,
    pub on_input: Option<Callback<(Id, String)>>,
}

enum PersonNameMsg {
    Input(String),
}

impl Component for PersonName {
    type Message = PersonNameMsg;
    type Properties = PersonNameProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            id: props.id,
            name: props.name.clone(),
            is_editting: props.is_editting,
            on_input: props.on_input,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PersonNameMsg::Input(n) => {
                debug!("input: {}, {}", self.id, self.name);
                if self.is_editting {
                    self.on_input.as_ref().map(|c| c.emit((self.id, n)));
                }
            }
        }

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let mut render = false;
        if self.is_editting != props.is_editting {
            self.is_editting = props.is_editting;
            render |= true;
        }

        if self.name != props.name {
            self.name = props.name;
            render |= true;
        }
        render
    }

    fn view(&self) -> Html {
        if self.is_editting {
            html! {
                <input type="text", value={&self.name}, oninput=self.link.callback(|e: InputData| PersonNameMsg::Input(e.value)), />
            }
        } else {
            html! {
                <>{ &self.name }</>
            }
        }
    }
}

#[derive(Clone)]
struct PersonAbility {
    id: Id,
    ability: Ability,
    is_editting: IsEditting,
    on_input: Option<Callback<(Id, Ability)>>,
    link: ComponentLink<Self>,
}
#[derive(Clone, PartialEq, Default, Properties)]
struct PersonAbilityProps {
    pub id: Id,
    pub ability: Ability,
    pub is_editting: IsEditting,
    pub on_input: Option<Callback<(Id, Ability)>>,
}

enum PersonAbilityMsg {
    Input(HtmlSelectElement),
}

impl Component for PersonAbility {
    type Message = PersonAbilityMsg;
    type Properties = PersonAbilityProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            id: props.id,
            ability: props.ability.clone(),
            is_editting: props.is_editting,
            on_input: props.on_input,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PersonAbilityMsg::Input(se) => {
                debug!("input: {}, {:?}", self.id, se.selected_index());

                let enum_i32: i32 = se.selected_index();
                let ability = Ability::from_i32(enum_i32);

                debug!("input: {}, {}", self.id, ability);
                if self.is_editting {
                    self.on_input.as_ref().map(|c| c.emit((self.id, ability)));
                }
            }
        }

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let mut render = false;
        if self.is_editting != props.is_editting {
            self.is_editting = props.is_editting;
            render |= true;
        }

        if self.ability != props.ability {
            self.ability = props.ability;
            render |= true;
        }
        render
    }

    fn view(&self) -> Html {
        if self.is_editting {
            let select_ability = |ability: Ability| {
                let value = i32::from(ability).to_string();
                if self.ability == ability {
                    html! {
                        <option value={value}, selected=true, >{ ability.to_str() }</option>
                    }
                } else {
                    html! {
                        <option value={value}, >{ ability.to_str() }</option>
                    }
                }
            };

            html! {
                <select onchange=self.link.callback(|e| match e {
                    ChangeData::Select(se) => PersonAbilityMsg::Input(se),
                    _ => unreachable!(),
                }),>
                   { for Ability::enumerate().iter().map(|a| select_ability(*a)) }
                </select>
            }
        } else {
            html! {
                <>{ self.ability.to_str() }</>
            }
        }
    }
}
