# checkni (Check No-Intro)
#### \\ 'chek-nē \\
: a small program used for verifying game dumps with No-Intro
### Why is this needed?
No-Intro's testimony explains it pretty well [here](https://no-intro.org/).

**TL;DR:** Preservation of data is important, and because game dumps are tampered with for various reasons it becomes harder to know if your copy is "legit". This program makes checking your game dumps easy and encourages people to search for those "original/unmodified dumps".
## Installation
#### Requirements
* Rust _(only needed if you are building from source)_
* A copy of the No-Intro database _(download that [here](https://datomatic.no-intro.org/index.php?page=download&op=daily) in `Standard DAT` form)_
* Your game dumps _(make sure they are in their standard dump type, not compressed into an archive)_

That's it! Just make sure the platforms you are checking dumps for are supported by No-Intro. _(you should be able to see what platforms are supported when you download the database file)_
#### Pre-compiled binary
1. Download checkni file from the [releases](https://github.com/dvcky/checkni/releases) page.
2. Move it to your desired location. **(preferably one where you have write permissions)**
#### Build from source
1. Clone the repository, `cd` into it, and build. (`cargo build --release`)
2. A binary will be created in `./target/release/` labeled `checkni` or `checkni.exe`.
3. Move it to your desired location. **(preferably one where you have write permissions)**

## FAQ
#### "I have a game dump that runs on my console/emulator fine, but checkni couldn't find it!"
That is most likely the releaser's intent. Many groups that release dumps often look for less noticable things they can take out of or add to the dump, which can be done for many reasons, such as conserving space or just generally "enhancing the user's experience" in some form. While this isn't necessarily bad on it's own, it does mean you have a file that is not the original.

_**TO CLARIFY:** This program is **NOT** meant for verifying that your game dump is "working" or "real", but rather that it is the **original/unmodified dump**._