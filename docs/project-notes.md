# Crab News

sketchpad for this project. notes, they come and go.

- [When Should You Use Which Collection?](https://doc.rust-lang.org/std/collections/index.html)

  > To get this out of the way: you should probably just use Vec or HashMap.
  > These two collections cover most use cases for generic data storage and
  > processing. They are exceptionally good at doing what they do. All the other
  > collections in the standard library have specific use cases where they are
  > the optimal choice, but these cases are borderline niche in comparison. Even
  > when Vec and HashMap are technically suboptimal, they're probably a good
  > enough choice to get started.

- Ports = capabilities, specifically the Operation types. They define the
  message interface with the shell
- Adapters = capability implementations outside the core (in Swift, Kotlin,
  TypeScript or even in Rust).

## TODO

- fix type aliases and
  [primitive obsession ](https://www.designisrefactoring.com/2016/08/31/refactoring-rust-primitive-obsession/)
- learn to use
  [newtypes](https://www.howtocodeit.com/articles/ultimate-guide-rust-newtypes)

## Elm vs Crux

To help wrap my head around it, hereby collected their similarities and
differences.

> [!NOTE]
>
> this can be updated once Crux Command API has been finalized. It would also be
> a good time to contribute to Crux docs.

- <https://redbadger.github.io/crux/guide/elm_architecture.html>

| Elm          | Crux                            | Notes                                                                                                                                     |
| ------------ | ------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| Model        | Model                           | the Model holds all the possible states the app can be in                                                                                 |
| a) View      | ViewModel                       | the ViewModel contains data relevant to the currently displayed UI/view                                                                   |
| b) View      | fn view() in App                | the fn view() function populates ViewModel's data from the Model                                                                          |
| c) View      | see "Cmd Msg" in Capabilities   | the Shells will send/receive the data to/from ViewModel via Capabilities                                                                  |
| d) View      | no Model -> Html Msg here       | unlike Elm, Crux doesn't render a View but sends data to Shells (see "c")                                                                 |
| Update       | fn update() in App              | takes Model, Events, Capabilities and changes Model by invoking Events                                                                    |
| a) Msg       | Events                          | are all the possible things the user can do                                                                                               |
| b) Cmd Msg   | Events                          | invoke Capabilities and may also callback more Events                                                                                     |
| (Model, Msg) | fn update() implicit return of? | self.update(Event::Update(count), model, caps);                                                                                           |
| main         | crux_core::App entry point      | is an implementation of the App trait, exposed via the Core or Bridge                                                                     |
| a) init      | ​#[derive(Default)]             | set Model initial state with [Default](https://doc.rust-lang.org/std/default/trait.Default.html) Trait; impl Default for any custom Types |
| b) init      | works like Elm's sandbox        | no request effects during init. You can always add Event::Init if needed                                                                  |

- <https://redbadger.github.io/crux/guide/capabilities.html>

| Side Effects    | Capabilities/FFI | Notes                                                                                   |
| --------------- | ---------------- | --------------------------------------------------------------------------------------- |
| a) Side Effects | Capabilities     | Crux has three types of effects: notifications, requests, and subscriptions             |
| b) Side Effects | Capabilities     | Crux side effects differ by the number of expected responses from the Shell             |
| c) Side Effects | Capabilities     | Crux fn update() in App is the only Capabilities consumer, via Events                   |
| Cmd Msg         | Capabilities     | from the perspective of the Shell, they are data oriented messages sent back and forth  |
| Cmd.none?       | Capabilities     | the Crux app will send the data to the Shell every time you call caps.render.render();  |
| subscriptions   | Capabilities     | subscriptions is a type of an effect in Crux, requested via capabilities                |
| ports           | Capabilities     | contrary to Elm Ports, Crux requests all side-effects, internally, through Capabilities |
| flags           | Event::Configure | favor something like Event::Configure to take the configuration options                 |

## Model

The Model is an overall state (and the only place for state) of your
application, it will hold all the loaded data, and any other kind of in-memory
cached things. Everything that needs to live for longer than single run of
`update()` goes in the Model.

- This needs some more love and thinking. It's a start though

```rust
#[derive(Default, Serialize)]
pub struct Model {
    ////////////////////////////
    // preferences UI
    ////////////////////////////
    theme: Theme,
    text_size: TextSize,
    browser: Browser,
    open_method: OpeningMethod,
    refresh_interval: RefreshInterval,
    // accounts: Vec<Account>, // will contain subscriptions in a future version

    ////////////////////////////
    // nain UI
    ////////////////////////////
    // "specials"
    unread_count: u16,
    starred_count: u16, // isUnread && isStarred
    entry_read: ReadStatus,
    entry_star: StarStatus,
    feed_url: String,

    // subscriptions,
    subscriptions: Subscriptions,
    subscription_folder: SubscriptionFolder, // root or folder

    // left column
    feed_view: FeedView, // Smart View = today | all unread | starred | folder | feed
    // for any account,
    account_name: String, // extrapolated from account
    feeds: Vec<Feed>,
    feed_name: String,

    // middle column
    entries_title: String, // folder or feed
    entries: Vec<Entry>,
    entry_title: String,
    entry_line: String, // whatever fits from content 1st line
    entry_date: Date, // dd mm yyyy

    // right column
    content: Option<Content>,
}
```

## ViewModel

the ViewModel is a straight "projection" of the Model -- it's calculated from it
(with the view function)

- This needs some more love and thinking. It's a start though

```rust
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ViewModel {
    ////////////////////////////
    // preferences UI
    ////////////////////////////
    pub theme: Theme,
    pub text_size: TextSize,
    pub use_browser: Browser,
    pub open_method: OpeningMethod,
    pub refresh_interval: RefreshInterval,
    // accounts: Vec<Account>, // contains subscriptions in the future

    ////////////////////////////
    // nain UI
    ////////////////////////////
    // "specials"
    pub subscriptions: OPML,
    pub unread_count: u16,
    pub starred_count: u16,
    pub entry_read: ReadStatus,
    pub entry_star: StarStatus,
    pub feed_url: String,

    // left column
    pub feed_view: FeedView, // Smart View: today | all unread | starred,
    // for any account,
    pub account_name: String, // extrapolated from account
    pub feed_store: FeedStore, // root or folder
    pub feed_name: String, // extrapolated from feed

    // middle column
    pub entries_title: String, // folder or feed
    pub entries: Vec<Entry>,
    pub entry_title: String,
    pub entry_line: String, // whatever fits from content 1st line
    pub entry_date: StarStatus, // dd mm yyyy

    // right column
    pub content: Option<Content>,

    ////////////////////////////
    // modals
    ////////////////////////////
    // subscribe modal
    pub feed_url: String,
    pub feed_name: String,
    pub feed_store: FeedStore,

    // delete feed/folder <T> modal
    pub app_logo: Image,
    pub del_title: String,
    pub del_what: String, // either feed_name or feed_store
    pub button_action: ,
}
```

## Account

- Do I need a crate here? Does Crux provide native integration?
  - <https://rclone.org> is interesting
- Likely needing to code my own Capability for this one?
  - <https://github.com/rust-lang/rust/issues/109381>
  - <https://developer.apple.com/documentation/uikit/documents_data_and_pasteboard/synchronizing_documents_in_the_icloud_environment>
- Probably best left for a future version?

> I don't think you need a crate here nor create a Capability. You can implement
> all inside the crux app and probably the only use crux_http and crux_kv (key
> value store) capabilities. You will use crux_http to communicate to the
> account clouds and probably the crux_kv to store the tokens locally. There are
> already examples on how to implement the crux_http on Android, iOS and the
> Web, but, I don't remember seeing any of the crux_kv shell implementations.

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Account {
    acct: AccountType,
    subs: Subscriptions,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub enum AccountType {
    #[default]
    Local(AccountLocal),
    Native(AccountNative),
    Cloud(AccountCloud),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum AccountLocal {
    Local { name: String, auth: bool },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum AccountNative {
    // how do I check for Auth? impl? Capabilities?
    Apple { name: String, auth: bool },
    Google { name: String, auth: bool },
    Microsoft { name: String, auth: bool },
    Canonical { name: String, auth: bool },
    // more?
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum AccountCloud {
    // https://rclone.org
    Dropbox { name: String, auth: bool },
    // more
}
```

## Subscriptions

- crate: <https://crates.io/crates/opml>
- <http://opml.org/spec2.opml>
- <http://outlinerhowto.opml.org>
- OPML crate to deal with subscriptions and outlines:

```rust
ImportSubscriptions(OpmlFile),
ExportSubscriptions(OpmlName),
AddNewFolder(FolderName),
DeleteFolder(FolderName),
RenameFolder(OldName, NewName),
AddNewSubscription(Option<FolderName>, SubscriptionName, SubscriptionURL),
DeleteSubscription(Option<FolderName>, SubscriptionName),
RenameSubscription(Option<FolderName>, OldName, NewName),
MoveSubscriptionToFolder(Subscription, OldFolder, NewFolder),
```

```xml
<!-- Example OPML -->
<?xml version="1.0" encoding="ISO-8859-1"?>
<!-- OPML generated by Crab News -->
<opml version="2.0">
  <head>
    <title>Subscriptions.opml</title>
    <dateCreated>Sat, 18 Jun 2005 12:11:52 GMT</dateCreated>
    <ownerName>Crab News</ownerName>
  </head>
  <body>
    <!-- this is an UNGROUPED root level subscription -->
    <outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/atom.xml"/>
    <!-- this is a GROUPED 1st level folder subscription -->
    <outline text="Group Name" title="Group Name">
      <outline text="Feed Name" title="Feed Name" description="" type="rss" version="RSS" htmlUrl="https://example.com/" xmlUrl="https://example.com/rss.xml"/>
    </outline>
  </body>
</opml>
```

## Feeds

- crate: <https://crates.io/crates/rss> to deal with RSS
- crate: <https://crates.io/crates/atom_syndication> to deal with Atom
- crate: <https://docs.rs/feed-rs/latest/feed_rs> handles both
- <https://datatracker.ietf.org/doc/html/rfc4287>
- <https://validator.w3.org/feed/docs/atom.html>
- <https://www.rssboard.org/rss-specification>

This crate is to deal with feeds data **after** subscribtions. The main UI would
deal with all data to display "news" in the entry and content columns.

### Related to Feeds

```rust
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub enum ReadStatus {
    Read,
    #[default]
    Unread,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub enum StarStatus {
    Starred,
    #[default]
    Unstarred,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub enum FeedView {
    Today,
    #[default]
    Unread,
    Starred,
    Folder,
    Feed,
}
```

## Events

- all the events to start coding, more later?

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    // EVENTS FROM THE SHELL
    // ANCHOR: Preferences UI
    // General panel
    SetTheme,
    SetTextSize,
    SetBrowser,
    SetOpeningMethod,
    SetRefreshInterval,
    // Account panel
    AddAccount,
    DeleteAccount,
    // ANCHOR_END: Preferences UI

    // ANCHOR: Menu
    // Shell thingiemageebs better done in UI?
    // File // mostly system related
    // Edit // mostly system related
    // View
    SortEntriesBy, // newest | oldest
    GroupByFeed,
    CleanUpEntries,
    HideRead // entries | feeds
    HideUIItem // sidebar | toolbar
    // Go
    DisplayNextUnreadEntry,
    DisplayToday,
    DisplayAllUnread,
    DisplayStarred,
    // Article -> SEE Entries
    // ANCHOR_END: Mmenu

    // ANCHOR: Main UI
    // Subscriptions live in struct Account {}
    ImportSubscriptions, // shows up in Menu -> File
    ExportSubscriptions, // shows up in Menu -> File

    // FeedStore -> root + 1st level folder. no more
    // THIS ADDS AN OUTLINE TO OPML::Body
    AddNewFolder, // shows up in Menu -> File
    DeleteFolder,
    RenameFolder,

    // FeedView -> today | all unread | starred | folder | feed
    SetFeedView,

    // Feeds
    // TO ADD AN OUTLINE TO ROOT USE https://docs.rs/opml/1.1.6/opml/struct.OPML.html#method.add_feed
    // TO ADD AN OUTLINE TO FOLDER USE https://docs.rs/opml/1.1.6/opml/struct.Outline.html#method.add_feed
    RefreshFeeds, // shows up in Menu -> File
    AddNewFeed, // account | root | folder // shows up in Menu -> File
    DeleteFeed,
    RenameFeed,
    MoveFeedToFolder, // location -> root | folder
    CopyFeedURL,
    CopyFeedHomeURL,
    OpenFeedHomeURL,

    // Entries
    MarkEntryAsRead, // shows up in Menu -> Article
    MarkEntryAsUnread, // shows up in Menu -> Article
    MarkAllEntriesAsRead, // shows up in Menu -> Article
    MarkAllEntriesAsUnread, // shows up in Menu -> Article
    MarkEntryAsStarred, // shows up in Menu -> Article
    MarkEntryAsUnstarred, // shows up in Menu -> Article
    OpenEntryInBrowser, // shows up in Menu -> Article
    CopyEntryURL,

    // Content has no Events associated but system ones

    // ANCHOR_END: Main UI

    // EVENTS LOCAL TO THE CORE
    #[serde(skip)]
    Fetch(crux_http::Result<crux_http::Response<Feed>, Box<dyn Error>>),
}
```

## Database

- Almost all data eventually goes into the db. adding as I go.
- crate: <https://crates.io/crates/surrealdb>
- embed: <https://surrealdb.com/docs/surrealdb/embedding/rust>
