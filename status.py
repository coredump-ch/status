#!/usr/bin/env python3.3
# coding=utf8

import cgi
import cgitb
import json

cgitb.enable()  # uncomment for debugging

data = {
    'api': '0.13',
    'space': 'coredump',
    'logo': 'http://www.coredump.ch/logo.png',
    'url': 'http://www.coredump.ch/',
    'location': {
        'address': 'Spinnereistrasse 2, 8640 Rapperswil, Switzerland',
        'lat': 47.22936,
        'lon': 8.82949
    },
    'spacefed': {
        'spacenet': False,
        'spacesaml': False,
        'spacephone': False
    },
    'state': {
        'open': False,
        'message': 'Open every monday from 18:00',
    },
    'contact': {
        'irc': 'irc://freenode.net/#coredump',
        'twitter': '@coredump_ch',
        'foursquare': '525c20e5498e875d8231b1e5',
        'email': 'danilo@coredump.ch',
    },
    'issue_report_channels': ['email', 'twitter'],
    'feeds': {
        'blog': {
            'type': 'rss',
            'url': 'http://www.coredump.ch/feed/',
        },
    },
    'cache': {
        'schedule': 'h.02',
    },
    'projects': ['https://github.com/coredump-ch/'],
}

print('Access-Control-Allow-Origin: *')
print('Cache-Control: no-cache')
print('Content-Type: application/json')
print()
print(json.dumps(data))
