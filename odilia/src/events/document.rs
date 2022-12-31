use zbus::zvariant::ObjectPath;
use odilia_cache::CacheItem;

use crate::state::ScreenReaderState;
use atspi::{
	events::GenericEvent,
	identify::DocumentEvents,
	identify::LoadCompleteEvent,
};

pub async fn load_complete(state: &ScreenReaderState, event: LoadCompleteEvent) -> eyre::Result<()> {
	let sender = event.sender()?.unwrap();
	let cache = state
		.build_cache(
			sender.clone(),
			ObjectPath::try_from("/org/a11y/atspi/cache".to_string())?,
		)
		.await?;
	let entire_cache = cache.get_items().await?;
	let mut cache_items = Vec::new();
	for item in entire_cache {
		cache_items.push(CacheItem {
			object: item.object.try_into().unwrap(),
			app: item.app.try_into().unwrap(),
			parent: item.parent.try_into().unwrap(),
			index: item.index,
			children: item.children,
			ifaces: item.ifaces,
			role: item.role,
			states: item.states,
			text: item.name.clone(),
		});
	}
	state.cache.add_all(cache_items).await;
	tracing::debug!("Add an entire document to cache.");
	Ok(())
}

pub async fn dispatch(state: &ScreenReaderState, event: DocumentEvents) -> eyre::Result<()> {
	// Dispatch based on member
	match event {
		DocumentEvents::LoadComplete(load_complete_event) => load_complete(state, load_complete_event).await?,
		other_member => tracing::debug!("Ignoring event with unknown member: {:#?}", other_member),
	}
	Ok(())
}
