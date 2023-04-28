use cursive::{
    event::{EventResult, Key},
    traits::With,
    view::{Nameable, scroll::Scroller, Scrollable},
    views::{Dialog, EditView, LinearLayout, ListView, OnEventView, TextView},
};
use cursive::Cursive;
use cursive::CursiveRunnable;

use crate::wordle;

pub struct Tui {
    siv: CursiveRunnable,
}

struct Data {
    solver: wordle::WordleWords,
}

impl Tui {
    pub fn new(solver: wordle::WordleWords) -> Tui {
        let mut siv = cursive::default();
        siv.set_user_data(Data {solver});
        Tui {
            siv: siv,
        }
    }

    pub fn start(&mut self) {
        // quit by pressing q
        self.siv.add_global_callback('q', |s| s.quit());

        // a scrollable view holding the current list of wordle words
        let wordle_list_view = Dialog::around(
            TextView::new(
                get_possible_words(&mut self.siv))
                .center()
                .with_name("wordlelist") // wrap in a NamedView
                .scrollable() // wrap in a ScrollView
                .wrap_with(OnEventView::new) // wrapper for reacting to events
                .on_pre_event_inner(Key::PageUp, |v, _| {
                    let scroller = v.get_scroller_mut();
                    if scroller.can_scroll_up() {
                        scroller.scroll_up(
                            scroller.last_outer_size().y.saturating_sub(1),
                        );
                    }
                    Some(EventResult::Consumed(None))
                })
                .on_pre_event_inner(Key::PageDown, |v, _| {
                    let scroller = v.get_scroller_mut();
                    if scroller.can_scroll_down() {
                        scroller.scroll_down(
                            scroller.last_outer_size().y.saturating_sub(1),
                        );
                    }
                    Some(EventResult::Consumed(None))
                })
        )
            .title("Possible")
            ;

        // a view holding the current list of excluded letters
        let exclude_letters_view = Dialog::around(
            TextView::new("")
                .center()
                .with_name("excludelist")
        )
            .title("Exclude")
            ;

        // a view holding the current list of included letters
        let include_letters_view = Dialog::around(
            TextView::new(".....")
                .center()
                .with_name("includelist")
        )
            .title("Include")
            ;

        // a view holding a list of incorrect letters (up to 5 entries)
        // for use with dordle, we allow up to 8 incorrect attempts
        let incorrect_letters_view = Dialog::around(
            ListView::new()
                .child("1", TextView::new(".....").with_name("incorrect0"))
                .child("2", TextView::new(".....").with_name("incorrect1"))
                .child("3", TextView::new(".....").with_name("incorrect2"))
                .child("4", TextView::new(".....").with_name("incorrect3"))
                .child("5", TextView::new(".....").with_name("incorrect4"))
                .child("6", TextView::new(".....").with_name("incorrect5"))
                .child("7", TextView::new(".....").with_name("incorrect6"))
                .child("8", TextView::new(".....").with_name("incorrect7"))
        )
            .title("Incorrect")
            .with_name("incorrectlist")
            ;

        // hold the exclude, include and incorrect letters list in a vertical view
        let linear_view_1 = LinearLayout::vertical()
            .child(exclude_letters_view)
            .child(include_letters_view)
            .child(incorrect_letters_view)
            ;

        // hold the previous view and the word list view in a horizontal view
        let linear_view_2 = LinearLayout::horizontal()
            .child(linear_view_1)
            .child(wordle_list_view)
            ;

        // surround the previous view with a dialog with options to update
        // the various views
        let dialog = Dialog::around(linear_view_2)
            .title("Wordle Words")
            .button("Exclude", |s| update_exclude(s))
            .button("Include", |s| update_include(s))
            .button("Incorrect", |s| update_incorrect(s))
            .button("Update", |mut s| {
                let exclude_content = s.call_on_name("excludelist",|view: &mut TextView| {
                    view.get_content()
                })
                    .unwrap();

                let include_content = s.call_on_name("includelist", |view: &mut TextView| {
                    view.get_content()
                })
                    .unwrap();

                let mut vec_list: Vec<String> = Vec::new();
                for index in 0..8 {
                    let name = &format!("incorrect{}", index);
                    let content = s.call_on_name(name, |view: &mut TextView| {
                        view.get_content()
                    })
                        .unwrap();
                    vec_list.push(content.source().to_string());
                }

                let data: &mut Data = s.user_data().unwrap();
                let words: &mut wordle::WordleWords = &mut data.solver;

                words.reset_list();
                words.remove_letters(&exclude_content.source());
                words.correct_letters(&include_content.source());
                for word in vec_list {
                    words.incorrect_letters(&word);
                }
                let new_list = get_possible_words(&mut s);
                s.call_on_name("wordlelist", |view: &mut TextView| {
                    view.set_content(new_list);
                });
            })
            .button("Reset", |s| reset(s))
            .button("Quit", |s| s.quit());

        self.siv.add_layer(dialog);

        self.siv.run();
    }
}

fn get_possible_words(siv: &mut Cursive) -> String {
    let data: &mut Data = siv.user_data().unwrap();
    let words: &wordle::WordleWords = &data.solver;

    let mut display: String = String::new();

    for word in words.get_word_list() {
        display.push_str(word);
        display.push('\n');
    }
    display
}

// display the excluded letters in a pop up for editing
fn update_exclude(siv: &mut Cursive) {
    // get the contents of the current exclude list
    let curr_content = siv.call_on_name("excludelist", |view: &mut TextView| {
        view.get_content()
    })
        .unwrap();
    // println!("content: {:?}", curr_content.source());

    // pop up an editview with the current contents for the user to edit
    siv.add_layer(
        Dialog::new()
            .title("Excluded Letters")
            .content(
                EditView::new()
                    .content(curr_content.source())
                    .with_name("new_exclude_list")
            )
            .button("Done", |s| {
                // get the new excluded letters
                let content = s
                    .call_on_name("new_exclude_list", |view: &mut EditView| view.get_content())
                    .unwrap();
                // println!("new content: {}", content);

                // update the exclude list
                s.call_on_name("excludelist", |view: &mut TextView| {
                    view.set_content(&*content)
                });
                s.pop_layer();
            })
            .button("Cancel", |s| {
                s.pop_layer();
            })
    );
}

// display the included letters in a pop up for editing
fn update_include(siv: &mut Cursive) {
    // get the contents of the current include list
    let curr_content = siv.call_on_name("includelist", |view: &mut TextView| {
        view.get_content()
    })
        .unwrap();
    // println!("content: {:?}", curr_content.source());

    // pop up an editview with the current contents for the user to edit
    siv.add_layer(
        Dialog::new()
            .title("Included Letters")
            .content(
                EditView::new()
                    .content(curr_content.source())
                    .max_content_width(5)
                    .with_name("new_include_list")
            )
            .button("Done", |s| {
                // get the new excluded letters
                let content = s
                    .call_on_name("new_include_list", |view: &mut EditView| view.get_content())
                    .unwrap();
                // println!("new content: {}", content);

                // update the exclude list
                s.call_on_name("includelist", |view: &mut TextView| {
                    view.set_content(&*content)
                });
                s.pop_layer();
            })
            .button("Cancel", |s| {
                s.pop_layer();
            })
    );
}

// display the list of incorrect letters in a pop up for editing
fn update_incorrect(siv: &mut Cursive) {
    let mut vec_list: Vec<String> = Vec::new();
    for index in 0..8 {
        let name = &format!("incorrect{}", index);
        let content = siv.call_on_name(name, |view: &mut TextView| {
            view.get_content()
        })
            .unwrap();
        vec_list.push(content.source().to_string());
    }
    // println!("content: {:?}", vec_list);

    // create a new list of editviews with the current incorrect items
    let mut listview = ListView::new();
    let mut index = 0;
    for item in vec_list {
        let new_name = &format!("new_incorrect{}", index);
        listview.add_child(
            &format!("{}", index + 1),
            EditView::new()
                .content(item)
                .max_content_width(5)
                .with_name(new_name)
        );
        index += 1;
    }
    // listview.with_name("new_incorrect_list");

    siv.add_layer(
        Dialog::new()
            .title("Incorrect Letters List")
            .content(
                listview.with_name("new_incorrect_list")
            )
            .button("Done", |s| {
                for index in 0..8 {
                    let new_name = &format!("new_incorrect{}", index);
                    let old_name = &format!("incorrect{}", index);
                    // get new content
                    let new_content = s
                        .call_on_name(new_name, |view: &mut EditView| view.get_content())
                        .unwrap();
                    // update old content with new content
                    s.call_on_name(old_name, |view: &mut TextView| {
                        view.set_content(&*new_content)
                    });
                }
                s.pop_layer();
            })
            .button("Cancel", |s| {
                s.pop_layer();
            })
    );
}

fn reset(siv: &mut Cursive) {
    // reset the list of words
    let data: &mut Data = siv.user_data().unwrap();
    let words: &mut wordle::WordleWords = &mut data.solver;
    words.reset_list();

    // reset the exclude list
    siv.call_on_name("excludelist", |view: &mut TextView| {
        view.set_content("")
    });
    // reset the correct list
    siv.call_on_name("includelist", |view: &mut TextView| {
        view.set_content(".....")
    });
    // reset the incorrect list
    for index in 0..8 {
        let name = &format!("incorrect{}", index);
        siv.call_on_name(name, |view: &mut TextView| {
            view.set_content(".....")
        });
    }
    // undate the possible word list
    let list = get_possible_words(siv);
    siv.call_on_name("wordlelist", |view: &mut TextView| {
        view.set_content(list);
    });
}
