use std::ops::DerefMut;

use stdweb::web::html_element::SelectElement;
use yew::callback::Callback;
use yew::format::Json;
use yew::prelude::*;

use kp_chart;
use kp_chart::data::*;
use web::*;

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
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PeopleStore {
    pub inc: usize,
    pub people: Vec<Person>,
}

impl PeopleStore {
    pub fn restore(context: &mut Context) -> Option<Self> {
        let from_store = context.local_store.restore(PEOPLE_KEY);
        match from_store {
            Json(Ok(people)) => Some(people),
            // TODO: reset local store...
            Json(Err(err)) => {
                context
                    .console
                    .error(&format!("could not load from local store: {}", err));
                None
            }
        }
    }

    pub fn store(&mut self, context: &mut Context) {
        self.inc += 1;
        context
            .console
            .debug(&format!("saving people: {}", self.inc));
        context.local_store.store(PEOPLE_KEY, Json(self as &Self));
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

impl From<PeopleStore> for PeopleModel {
    fn from(model: PeopleStore) -> Self {
        Self {
            inc: model.inc,
            people: model.people.into_iter().map(|p| (p, false)).collect(),
        }
    }
}

impl Component<Context> for PeopleModel {
    type Message = PeopleMsg;
    type Properties = ();

    fn create(_props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        context.console.debug("creating PeopleModel");

        match PeopleStore::restore(context.deref_mut()) {
            Some(this) => this.into(),
            None => {
                let people = kp_chart::default_people();
                // TODO: make a borrowed type
                let mut people = PeopleStore {
                    inc: 0,
                    people: people,
                };

                people.store(context.deref_mut());
                people.into()
            }
        }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            PeopleMsg::SavePeople => {
                context.console.debug("saving PeopleModel");
                let mut people: PeopleStore = self.clone().into();
                people.store(context.deref_mut());

                for (_, editting) in self.people.iter_mut() {
                    *editting = false;
                }
                true
            }
            PeopleMsg::AddPerson => {
                context.console.debug("adding a Person");
                let person = Person::new("Jane Doe", Ability::Adult);
                self.people.push((person, true));
                true
            }
            PeopleMsg::EditPerson(id) => {
                context.console.debug(&format!("edit person: {}", id));
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
                context.console.debug(&format!("deleted {:?}", person));
                true
            }
            PeopleMsg::PersonNameInput(id, name) => self
                .people
                .get_mut(id)
                .map(|p| {
                    context.console.debug(&format!("saving name: {}", name));
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
                    context.console.debug(&format!("saving name: {}", ability));
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
}

impl Renderable<Context, PeopleModel> for PeopleModel {
    fn view(&self) -> Html<Context, Self> {
        // let select = |is_selected: bool| {
        //     html!{
        //         <Select: is_selected={is_selected}, />
        //     }
        // };

        let edit_delete = |id: Id, is_editting: IsEditting| {
            html!{
                <EditDelete: id={id}, is_editting={is_editting}, on_edit=|id: Id| PeopleMsg::EditPerson(id), on_delete=|id: Id| PeopleMsg::DeletePerson(id), />
            }
        };
        let person_row = |id: Id, person: &(Person, IsEditting)| {
            html!{
                <tr>
                    <td><PersonName: id={id}, name={person.0.name().clone()}, is_editting={person.1}, on_input=|(i,n)| PeopleMsg::PersonNameInput(i, n),/></td>
                    <td><PersonAbility: id={id}, ability={person.0.ability()}, is_editting={person.1}, on_input=|(i,a)| PeopleMsg::PersonAbilityInput(i, a),/></td>
                    <td class="edit_delete",>{ edit_delete(id, person.1) }</td>
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
                        { for self.people.iter().enumerate().map(|(i, p)| person_row(i, p)) }
                    </tbody>
                    <tfoot>
                        <tr><td>
                            <button onclick=|_| PeopleMsg::AddPerson, >
                                <i class=("fa", "fa-plus-square"), aria-hidden="true",></i>
                            </button>
                            <button onclick=|_| PeopleMsg::SavePeople, >
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
#[derive(Clone, PartialEq, Default)]
struct EditDelete {
    pub id: Id,
    pub is_editting: IsEditting,
    pub on_edit: Option<Callback<Id>>,
    pub on_delete: Option<Callback<Id>>,
}

enum EditDeleteMsg {
    Edit,
    Delete,
}

impl Component<Context> for EditDelete {
    type Message = EditDeleteMsg;
    type Properties = Self;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {
        Self {
            id: props.id,
            is_editting: props.is_editting,
            on_edit: props.on_edit,
            on_delete: props.on_delete,
        }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            EditDeleteMsg::Edit => {
                context.console.debug(&format!("editting: {}", self.id));
                if !self.is_editting {
                    self.on_edit.as_ref().map(|c| c.emit(self.id));
                }
            }
            EditDeleteMsg::Delete => {
                context.console.debug(&format!("deleting: {}", self.id));
                self.on_delete.as_ref().map(|c| c.emit(self.id));
            }
        }

        false
    }

    fn change(
        &mut self,
        props: Self::Properties,
        _context: &mut Env<Context, Self>,
    ) -> ShouldRender {
        if self.is_editting != props.is_editting {
            self.is_editting = props.is_editting;
            return true;
        }
        false
    }
}

impl Renderable<Context, EditDelete> for EditDelete {
    fn view(&self) -> Html<Context, Self> {
        let disabled = if self.is_editting { "disabled" } else { "" };

        html! {
            <div class="edit_delete", >
                <i class=("fa", "fa-pencil-square-o", "fa-fw", disabled), aria-hidden="true", onclick=|_| EditDeleteMsg::Edit, />
                <i class=("fa", "fa-trash", "fa-fw"), aria-hidden="true", onclick=|_| EditDeleteMsg::Delete, />
            </div>
        }
    }
}

#[derive(Clone, PartialEq, Default)]
struct PersonName {
    pub id: Id,
    pub name: String,
    pub is_editting: IsEditting,
    pub on_input: Option<Callback<(Id, String)>>,
}

enum PersonNameMsg {
    Input(String),
}

impl Component<Context> for PersonName {
    type Message = PersonNameMsg;
    type Properties = Self;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {
        Self {
            id: props.id,
            name: props.name.clone(),
            is_editting: props.is_editting,
            on_input: props.on_input,
        }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            PersonNameMsg::Input(n) => {
                context
                    .console
                    .debug(&format!("input: {}, {}", self.id, self.name));
                if self.is_editting {
                    self.on_input.as_ref().map(|c| c.emit((self.id, n)));
                }
            }
        }

        false
    }

    fn change(
        &mut self,
        props: Self::Properties,
        _context: &mut Env<Context, Self>,
    ) -> ShouldRender {
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
}

impl Renderable<Context, PersonName> for PersonName {
    fn view(&self) -> Html<Context, Self> {
        if self.is_editting {
            html! {
                <input type="text", value={&self.name}, oninput=|e| PersonNameMsg::Input(e.value), />
            }
        } else {
            html! {
                <>{ &self.name }</>
            }
        }
    }
}

#[derive(Clone, PartialEq, Default)]
struct PersonAbility {
    pub id: Id,
    pub ability: Ability,
    pub is_editting: IsEditting,
    pub on_input: Option<Callback<(Id, Ability)>>,
}

enum PersonAbilityMsg {
    Input(SelectElement),
}

impl Component<Context> for PersonAbility {
    type Message = PersonAbilityMsg;
    type Properties = Self;

    fn create(props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {
        Self {
            id: props.id,
            ability: props.ability.clone(),
            is_editting: props.is_editting,
            on_input: props.on_input,
        }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            PersonAbilityMsg::Input(se) => {
                context
                    .console
                    .debug(&format!("input: {}, {:?}", self.id, se.selected_index()));

                let enum_i32: i32 = se.selected_index().expect("ability needs to be set") as i32;
                let ability = Ability::from_i32(enum_i32);

                context
                    .console
                    .debug(&format!("input: {}, {}", self.id, ability));
                if self.is_editting {
                    self.on_input.as_ref().map(|c| c.emit((self.id, ability)));
                }
            }
        }

        false
    }

    fn change(
        &mut self,
        props: Self::Properties,
        _context: &mut Env<Context, Self>,
    ) -> ShouldRender {
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
}

impl Renderable<Context, PersonAbility> for PersonAbility {
    fn view(&self) -> Html<Context, Self> {
        if self.is_editting {
            let select_ability = |ability: Ability| {
                // TODO: figure out selected
                html! {
                    <option value={i32::from(ability)}, >{ ability.to_str() }</option>
                }
            };

            html! {
                <select onchange=|e| match e {
                    ChangeData::Select(se) => PersonAbilityMsg::Input(se),
                    _ => unreachable!(),
                },>
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
