# How to contribute to CuteKit?
## Steps for creating good issues or pull requests.
If your issue is related to a bug, please give the full error message if one is shown. Please describe the context of the bug, the OS version, the calculator model and if possible, the release version or the latest commit.

If you are asking for an enhancement, please consider the following hardware restrictions:
- No wireless
- No sound
- CuteKit must be compatible with **N0110**, **N0115** and **N0120**. So make sure that the lib can be run on every model.
- The entire application only has around **100kB of RAM available**, for both Cutekit **and** the program that uses it.

## About vibe coding and AI

Every AI has ethical issues: crawling, copyright, environment, advertising etc...

CuteKit is a **human first** project.
One goal is to show what human devs are capable of, not what the latest *Fable 19 pro plus extreme* can do.
Moreover, most of the code that AI produces is not optimized for the calculator or is full of bugs.

For thoses reasons, **AI-programmed contributions are forbidden**.

However, help from an AI to find a bug in your own code is tolerated as long as you disclose what the AI was used for.

---

## Code guidelines

- Your code (functions, variables, comments, etc...) must be written in English.

- Name your variables explicitely (no shortened words or single letters unless it is easy to tell what the variable does)

- API exposed functions must be documented. Don't be afraid to comment your code in general.

- Do not refactor a major part of the project unprompted. Please ask us first.

- Use the correct module for your features. Feel free to make your own if what you want to implement is not closely related to an existing one.

- Always try your code on real hardware.
