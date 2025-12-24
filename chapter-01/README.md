# Chapter 01: The Theater Company

Imagine a theater company whose actors frequently perform in various venues. Typically, a customer specifies a set of plays, and the company charges the customer based on the audience size and the type of each play. Currently, the company performs two types of plays: tragedies and comedies. When issuing invoices to customers, the company also provides "volume credits" based on the number of audience members—these credits can be used for discounts on future performances, effectively serving as a customer loyalty incentive.

The company stores its play data in a simple JSON file.

[plays.json...](demo-1.1-1/plays.json)

The company also stores invoice data in a JSON file.

[invoices.json...](demo-1.1-1/invoices.json)

The function `statement` is responsible for generating the invoice details.
