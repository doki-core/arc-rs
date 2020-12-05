[
    {
        "invoice": Number {
            handler: None,
            value: InlineInteger(
                34843,
            ),
        },
        "date": "2001-01-23",
        "bill-to": {
            "given": "Chris",
            "family": "Dumars",
            "address": {
                "lines": "458 Walkman Dr.
                Suite #292
                ",
                "city": "Royal Oak",
                "state": "MI",
                "postal": Number {
                    handler: None,
                    value: InlineInteger(
                        48046,
                    ),
                },
            },
        },
        "ship-to": {
            "given": "Chris",
            "family": "Dumars",
            "address": {
                "lines": "458 Walkman Dr.
                Suite #292
                ",
                "city": "Royal Oak",
                "state": "MI",
                "postal": Number {
                    handler: None,
                    value: InlineInteger(
                        48046,
                    ),
                },
            },
        },
        "product": [
            {
                "sku": "BL394D",
                "quantity": Number {
                    handler: None,
                    value: InlineInteger(
                        4,
                    ),
                },
                "description": "Basketball",
                "price": Number {
                    handler: None,
                    value: InlineDecimal(
                        450.0,
                    ),
                },
            },
            {
                "sku": "BL4438H",
                "quantity": Number {
                    handler: None,
                    value: InlineInteger(
                        1,
                    ),
                },
                "description": "Super Hoop",
                "price": Number {
                    handler: None,
                    value: InlineDecimal(
                        2392.0,
                    ),
                },
            },
        ],
        "tax": Number {
            handler: None,
            value: InlineDecimal(
                251.42,
            ),
        },
        "total": Number {
            handler: None,
            value: InlineDecimal(
                4443.52,
            ),
        },
        "comments": "Late afternoon is best. Backup contact is Nancy Billsmer @ 338-4338.",
    },
    {
        "Time": "2001-11-23 15:01:42 -5",
        "User": "ed",
        "Warning": "This is an error message for the log file",
    },
    {
        "Time": "2001-11-23 15:02:31 -5",
        "User": "ed",
        "Warning": "A slightly different error message.",
    },
    {
        "Date": "2001-11-23 15:03:17 -5",
        "User": "ed",
        "Fatal": "Unknown variable "bar"",
        "Stack": [
            {
                "file": "TopClass.py",
                "line": Number {
                    handler: None,
                    value: InlineInteger(
                        23,
                    ),
                },
                "code": "x = MoreObject("345\n")
                ",
            },
            {
                "file": "MoreClass.py",
                "line": Number {
                    handler: None,
                    value: InlineInteger(
                        58,
                    ),
                },
                "code": "foo = bar",
            },
        ],
    },
]