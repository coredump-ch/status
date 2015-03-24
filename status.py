# -*- coding: utf-8 -*-
import json

from bottle import default_app, route, response, run, request


app = default_app()


@route('/')
def json_out():
    response.set_header('Access-Control-Allow-Origin', '*')
    response.set_header('Cache-Control', 'no-cache')
    response.set_header('Content-Type', 'application/json')

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
            'open': False,
            'message': 'Open every monday from 20:00',
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
        'projects': [
            'https://www.coredump.ch/projekte/',
            'https://discourse.coredump.ch/c/projects',
            'https://github.com/coredump-ch/',
        ],
        'sensors': {
            'people_now_present': {
                'value': 0,
            },
        },
    }

    people = None
    try:
        with open('people.txt', 'r') as f:
            people = int(f.read().strip())
    except:
        pass
    if people and people > 0:
        data['state']['open'] = True
        data['sensors']['people_now_present']['value'] = people
        base_msg = '1 person' if people == 1 else '%s people' % people
        data['state']['message'] = base_msg + ' present right now'

    return json.dumps(data)


@route('/update', method='POST')
def update():
    """
    Update the data in a text file.
    TODO: Public / Private key crypto.
    """
    people = int(request.POST.get('people'))
    with open('people.txt', 'w') as f:
        f.write(str(people))
    return 'OK'


if __name__ == '__main__':
    run(host='localhost', port=8080)
