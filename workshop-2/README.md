# Rust workshop 2 - Rust in the (more) real world

This is the second session of the rust workshop, meant to extend upon things learnt from the first workshop. This workshop covers more real life use cases such as webserver & ui's.

## Part 1: Quick refresher on networking & serialization

Let's have a quick refresher on networking with rust! Last workshop we covered how to send GET request to a server (api). Let's go through that again.


We also used the `serde` library for the deserialization of the payload too, like so!

Do try it if you want to, just to refresh your memory on this.


With that done let's try to send a different type of request: a POST request. POST request is a pretty common request that is frequently used to send data that mutates the server state to the server. Again we can do the same thing as the GET request, but this time we will also need to pass in a `body`.

This is where `serde`'s serialization comes in handy.


Now, let's send the request out, remember to also define the shape of the response and use the `serde:Deserialize` macro to tell it to deserialize it to a rust data structure.

It should now look like this!

Congrats, you now have a tool to talk to servers all around the world.

## Part 2: Building a Http web server from scratch

Now, let's make a http server that can let other people communicate with us.

## Part 3: Building a Http web server with a web library

In the real world, you usually would 


## Part 4: Building a p2p application

