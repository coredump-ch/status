Space API Implementation
========================

This is an implementation of the `SpaceAPI <http://spaceapi.net/>`_.

API Documentation: http://spaceapi.net/documentation

Development
-----------

Create a Python 2 Virtualenv (see
https://blog.dbrgn.ch/2012/9/18/virtualenv-quickstart/). Then install Bottle::

    pip install -r requirements.txt

Now you can start the development server directly::

    python status.py

Deployment
----------

For deployment, it's better to use a proper WSGI web server instead of the
single threaded dev server, for example Gunicorn::

    pip install gunicorn
    gunicorn -b 127.0.0.1:9002 -w 4 status:app

This will start a multithreaded server with 4 workers on port 9002.

License
-------

MIT, see ``LICENSE.txt``.
