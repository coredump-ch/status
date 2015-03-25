# -*- coding: utf-8 -*-
import json

from bottle import default_app, route, response, run, request


app = default_app()


def get_number_of_people():
    """
    Return an integer or None.
    """
    people = None
    try:
        with open('people.txt', 'r') as f:
            people = int(f.read().strip())
    except:
        pass
    return people


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
            'schedule': 'm.02',
        },
        'projects': [
            'https://www.coredump.ch/projekte/',
            'https://discourse.coredump.ch/c/projects',
            'https://github.com/coredump-ch/',
        ],
        'sensors': {
            'people_now_present': [
                {'value': 0},
            ],
        },
    }

    people = get_number_of_people()
    if people and people > 0:
        data['state']['open'] = True
        data['sensors']['people_now_present'][0]['value'] = people
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


@route('/sensors/people_now_present/html')
def sensor_people_html():
    """
    Show a HTML version of the opening status.
    """
    people = get_number_of_people()
    html = []
    if people and people > 0:
        html.append('<p class="spaceapi opening_status">Open</p>')
        msg = '1 person' if people == 1 else '%s people' % people
        html.append('<p class="spaceapi people">%s present</p>' % msg)
    else:
        html.append('<p class="spaceapi opening_status">Closed</p>')
    return '\n'.join(html)


if __name__ == '__main__':
    run(host='localhost', port=8080)
