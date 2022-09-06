use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use bevy::prelude::{Component, Entity, EventWriter, Mut, Query, With};

use crate::{
    find_entity,
    node::{Finals, UpdateEvent},
    CommonNode, Node, TypedNode,
};

pub struct Fetch<'a, T> {
    node: &'a Node,
    _type_marker: PhantomData<T>,
}

pub type FetchQuery<'world, 'state, 'b, N> = Query<'world, 'state, &'b Node, With<<N as TypedNode>::Type>>;

impl<'a, T> Fetch<'a, T>
where
    T: TypedNode + 'static,
    <T as TypedNode>::Type: Component,
{
    pub fn find(name: impl AsRef<str>, query: &'a FetchQuery<T>) -> Option<Self> {
        let id = find_entity(name)?;
        Self::new(id, query)
    }

    pub fn new(id: Entity, query: &'a FetchQuery<T>) -> Option<Self> {
        let node = query.get(id).ok()?;
        node.as_any_ref().downcast_ref::<&T>()?;

        Some(Self {
            node,
            _type_marker: PhantomData::default(),
        })
    }

    pub fn cast(&self) -> &T {
        self.node.as_any_ref().downcast_ref().unwrap()
    }
}

impl<T: TypedNode + 'static> Deref for Fetch<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.node.as_any_ref().downcast_ref().unwrap()
    }
}

pub struct FetchMut<'world, 'state, T> {
    finals_updater: Option<EventWriter<'world, 'state, UpdateEvent>>,
    node: Mut<'world, Node>,
    finals: &'world Finals,
    _type_marker: PhantomData<T>,
}

pub type FetchMutQuery<'world, 'state, 'a, 'b, N> =
    Query<'world, 'state, (&'a mut Node, &'b Finals), With<<N as TypedNode>::Type>>;

impl<'world, 'state, T> FetchMut<'world, 'state, T>
where
    T: TypedNode + 'static,
    <T as TypedNode>::Type: Component,
{
    pub fn find(name: impl AsRef<str>, query: &'world mut FetchMutQuery<T>) -> Option<Self> {
        let id = find_entity(name)?;
        Self::new(id, query)
    }

    pub fn find_with_finals_updater(
        name: impl AsRef<str>,
        query: &'world mut FetchMutQuery<T>,
        updater: EventWriter<'world, 'state, UpdateEvent>,
    ) -> Option<Self> {
        Some(Self::find(name, query)?.with_finals_updater(updater))
    }

    pub fn new(id: Entity, query: &'world mut FetchMutQuery<T>) -> Option<Self> {
        let (node, finals) = query.get_mut(id).ok()?;
        node.as_any_ref().downcast_ref::<T>()?;

        Some(Self {
            finals_updater: None,
            node,
            finals,
            _type_marker: PhantomData::default(),
        })
    }

    pub fn new_with_finals_updater(
        id: Entity,
        query: &'world mut FetchMutQuery<T>,
        updater: EventWriter<'world, 'state, UpdateEvent>,
    ) -> Option<Self> {
        Some(Self::new(id, query)?.with_finals_updater(updater))
    }

    pub fn with_finals_updater(mut self, updater: EventWriter<'world, 'state, UpdateEvent>) -> Self {
        self.finals_updater.replace(updater);
        self
    }

    pub fn cast(&mut self) -> &mut T {
        self.node.as_any_mut().downcast_mut().unwrap()
    }
}

impl<'world, 'state, T> FetchMut<'world, 'state, T> {
    pub fn finalize(&mut self) -> Option<EventWriter<'world, 'state, UpdateEvent>> {
        self.finals_updater.take().map(|mut event_writer| {
            for final_id in &self.finals.0 {
                event_writer.send(UpdateEvent(*final_id));
            }
            event_writer
        })
    }
}

impl<T> Drop for FetchMut<'_, '_, T> {
    fn drop(&mut self) {
        self.finalize();
    }
}

impl<T: TypedNode + 'static> Deref for FetchMut<'_, '_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.node.as_any_ref().downcast_ref().unwrap()
    }
}

impl<T: TypedNode + 'static> DerefMut for FetchMut<'_, '_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.node.as_any_mut().downcast_mut().unwrap()
    }
}

pub trait FetchingNode: TypedNode + Sized + 'static
where
    <Self as TypedNode>::Type: Component,
{
    fn get<'a>(id: Entity, query: &'a FetchQuery<Self>) -> Option<Fetch<'a, Self>> {
        Fetch::new(id, query)
    }

    fn get_mut<'world, 'state>(
        id: Entity,
        query: &'world mut FetchMutQuery<Self>,
        updater: EventWriter<'world, 'state, UpdateEvent>,
    ) -> Option<FetchMut<'world, 'state, Self>> {
        FetchMut::new_with_finals_updater(id, query, updater)
    }

    fn find<'a>(name: impl AsRef<str>, query: &'a FetchQuery<Self>) -> Option<Fetch<'a, Self>> {
        Fetch::find(name, query)
    }

    fn find_mut<'world, 'state>(
        name: impl AsRef<str>,
        query: &'world mut FetchMutQuery<Self>,
        updater: EventWriter<'world, 'state, UpdateEvent>,
    ) -> Option<FetchMut<'world, 'state, Self>> {
        FetchMut::find_with_finals_updater(name, query, updater)
    }
}

impl<T> FetchingNode for T
where
    T: TypedNode + 'static,
    <T as TypedNode>::Type: Component,
{
}
