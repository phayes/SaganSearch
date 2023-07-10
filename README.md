# SaganSearch

### Searching for artifical messages in the digits of pi

This is an art project inspired by the final chapter of [Carl Sagan's *Contact*](https://en.wikipedia.org/wiki/Contact_(novel)). In the book, the main character discovers artificial messages encoded within transcendental numbers like π. This project aims to replicate that fictional search in reality.

SaganSearch works by looking for regions in π that contain relatively low entropy as measured by the [Shannon Entropy Metric](https://en.wikipedia.org/wiki/Entropy_(information_theory)). The search has been started at 100 trillion digits into π and is working backwards towards the beginning 1000 digits at a time. SaganSearch uses the 100 trillion digits of pi generated by [Google Cloud](https://cloud.google.com/blog/products/compute/calculating-100-trillion-digits-of-pi-on-google-cloud).

## Art

This is an abstract art project that encourages thought about the fundemental nature of reality and challenges the artifical cultural separation between creationism and science. If a message is found, it could mean our reality was created by an intelligent mind. It's a hail-mary search for God. 

## FAQ

**1. Why π and not other numbers?**

π has not yet been proven to be [normal](https://en.wikipedia.org/wiki/Normal_number) and has been generated out to 100 trillion digits; it provides a great place to start the search. Any transcendental number that has not been proven normal would be a good candidate for a search (for example [Euler's Number](https://en.wikipedia.org/wiki/E_(mathematical_constant))). Looking beyond transcendental numbers, other fundemental constructs like the [Monster Group](https://en.wikipedia.org/wiki/Monster_group) could be a fun place to look for signs of artificiality.

**2. What would it mean if a message is found?**

The implications of a message would be profound. 

In addition to the meaning of the message itself, a message in π would could mean that our reality was created by an intelligent mind. It could also mean that fundemental truths are in a certain sense arbitrary and artificial. It could mean that there are alternative realities out there where `1 + 1 = 3` despite this statement being totally nonsense within our reality. The implications on our understanding of reality would be profound.

**3. Do you really expect to find a message?**

Honestly, not really. This is framed as an art project because it's primary purpose is to provoke interesting thoughts about the nature of reality. But who knows, maybe a message really is there, deep in the digits of pi. 

## Joining the search

If you decide you want to run SaganSearch, please email me at patrick.d.hayes@gmail.com so we can coodinate different search spaces within the digits of π.  I am also happy to accept pull requests that expand the project to other transcendental numbers, supports an organized distributed search, or any other improvements.

## Future development plans

1. Use direct ycd files instead of the web API for fetching digits
2. Turn SaganSearch into a proper distributed system
3. Explore other ways of looking for signal other than the Shannon Entropy Metric
4. Refine entropy cut-off point (currently set to 0.9)
5. Add Euler's Number and other transendentals. 

## Results

Intial results of the search will be published on Mar 14, 2024. Email patrick.d.hayes@gmail.com if you're interested in getting an email announcement of the findings.
