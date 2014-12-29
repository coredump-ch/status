# -*- coding: utf-8 -*-
import json

from bottle import default_app, route, response, run


app = default_app()


data = {
    'api': '0.13',
    'space': 'coredump',
    'logo': 'https://www.coredump.ch/logo.png',
    'url': 'https://www.coredump.ch/',
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
        'open': None,
        'message': 'Open every monday from 19:30',
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
            'url': 'https://www.coredump.ch/feed/',
        },
    },
    'cache': {
        'schedule': 'h.02',
    },
    'projects': ['https://github.com/coredump-ch/'],
}


@route('/')
def json_out():
    response.set_header('Access-Control-Allow-Origin', '*')
    response.set_header('Cache-Control', 'no-cache')
    response.set_header('Content-Type', 'application/json')
    return json.dumps(data)


if __name__ == '__main__':
    run(host='localhost', port=8080)
