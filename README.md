# ballotbox

I'm working on developing a simple voting application in Rust implemented as a TELNET server 😂 just for fun. You can connect to a server which can start a new vote, and you get to pick and after a certain time the results are broadcasted to everybody!

Inspired by [Tsoding's `Multiuser Chat`` series](https://www.youtube.com/watch?v=BbIEuNscn_E) and the November 2023 Dutch elections.

**_WORK IN PROGRESS_**

## TODO

- [x] The admin should be presented with a terminal interface
- [] When a user connects, they should enter their name, this will be recorded into the "people" state
- [] The admin should be able to view results of old / the current vote
- [] The admin should be able to create a new vote, with options
- [] The admin should be able to kick / ban voters
- [] The admin should be able to adjust settings, such as toggling the banlist.
