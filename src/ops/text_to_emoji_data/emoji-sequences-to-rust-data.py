#!/usr/bin/env python3

"""
Convert Emoji Sequence Data for UTS #51 into rust code.
In particular, I need the name-to-emoji association.

Be aware that the file is incomplete in the sense that
ranges are given where the individual emoji are not listed.
Hence, python's unicodedata module is utilized as well

Invoked with: python3 emoji-sequences-to-rust-data.py emoji-sequences.txt emoji-data.bin
"""

import os
import re
import sys
import logging
import datetime
import argparse
import unicodedata

LOGGER = logging.getLogger(__name__)

def setup(loglevel, logfmt):
    logging.basicConfig(level=loglevel, format=logfmt)
    LOGGER.setLevel(loglevel)

# this data is not included in my python 3.10.12 version's unicodedata module
additional_unicodedata = {
    0x1F6DD: 'playground slide',
    0x1F6DE: 'wheel',
    0x1F6DF: 'ring buoy',
    0x1FA75: 'light blue heart',
    0x1FA76: 'grey heart',
    0x1FA77: 'pink heart',
    0x1FA7B: 'x-ray',
    0x1FA7C: 'crutch',
    0x1FA87: 'maracas',
    0x1FA88: 'flute',
    0x1FAA9: 'mirror ball',
    0x1FAAA: 'identification card',
    0x1FAAB: 'low battery',
    0x1FAAC: 'hamsa',
    0x1FAAD: 'folding hand fan',
    0x1FAAE: 'hair pick',
    0x1FAAF: 'khanda',
    0x1FAB7: 'lotus',
    0x1FAB8: 'coral',
    0x1FAB9: 'empty nest',
    0x1FABA: 'nest with eggs',
    0x1FABB: 'hyacinth',
    0x1FABC: 'jellyfish',
    0x1FABD: 'wing',
    0x1FAC3: 'pregnant man',
    0x1FAC4: 'pregnant person',
    0x1FAC5: 'person with crown',
    0x1FACE: 'moose',
    0x1FACF: 'donkey',
    0x1FAD7: 'pouring liquid',
    0x1FAD8: 'beans',
    0x1FAD9: 'jar',
    0x1FADA: 'ginger root',
    0x1FADB: 'pea pod',
    0x1FAE0: 'melting face',
    0x1FAE1: 'saluting face',
    0x1FAE2: 'face with open eyes and hand over mouth',
    0x1FAE3: 'face with peeking eye',
    0x1FAE4: 'face with diagonal mouth',
    0x1FAE5: 'dotted line face',
    0x1FAE6: 'biting lip',
    0x1FAE7: 'bubbles',
    0x1FAF0: 'hand with index finger and thumb crossed',
    0x1FAF1: 'rightwards hand',
    0x1FAF2: 'leftwards hand',
    0x1FAF3: 'palm down hand',
    0x1FAF4: 'palm up hand',
    0x1FAF5: 'index pointing at the viewer',
    0x1FAF6: 'heart hands',
    0x1FAF7: 'leftwards pushing hand',
    0x1FAF8: 'rightwards pushing hand',
}


entry_regex = re.compile(
    r'((?P<scalarrange>[0-9A-F]{4,8}\.\.[0-9A-F]{4,8})|(?P<scalarmulti>[0-9A-F]{4,8}( [0-9A-F]{4,8})*))'
    r'\s*;\s*'
    r'(?P<typefield>\S+)'
    r'\s*;\s*'
    r'(?P<description>[^#]+?)\s*'
    r'(#\s*E(?P<version>([0-9.]+)))'
    r'\s+'
    r'(\[(?P<scalarscount>[0-9]+)\])'
    r'\s+'
    r'\(((?P<emojirange>[^).]+\.\.[^)]+)|(?P<emoji>[^).]+))\)'
    r'\s*'
)

def iterate(src_filepath: str):
    """Iterate extended emoji data listed in the UTS file"""
    with open(src_filepath) as fd:
        for line in fd:
            if not line.strip() or line.startswith('#'):
                continue

            m = entry_regex.match(line)
            if m is None:
                raise ValueError(f"Could not read line with regular expression - line: {line!r}")

            data = m.groupdict()
            if data['scalarrange']:
                first, last = data['scalarrange'].split('..')
                first, last = int(first, 16), int(last, 16)
                for scalar in range(first, last + 1):
                    try:
                        name = unicodedata.name(chr(scalar)).lower()
                    except ValueError:
                        name = additional_unicodedata[scalar]
                    yield {'scalars': [chr(scalar)], 'description': name, 'emoji': chr(scalar)}
            else:
                assert data['scalarmulti']
                #print(data, hex(scalar))
                yield {
                    'scalars': [chr(int(s, 16)) for s in data['scalarmulti'].split()],
                    'description': data['description'],
                    'emoji': data['emoji'],
                }

def main(src_filepath, dst_filepath, dst_overwrite):
    """Main routine"""
    with open(dst_filepath, 'wb' if dst_overwrite else 'xb') as fd:
        for entry in iterate(src_filepath):
            utf8_encoding = b''.join([s.encode('utf-8') for s in entry['scalars']])
            description = entry['description'].encode('utf-8')

            fd.write(utf8_encoding)
            fd.write(b'\x1F')  # U+001F UNIT SEPARATOR
            fd.write(description)
            fd.write(b'\x1E')  # U+001E RECORD SEPARATOR


if __name__ == '__main__':
    loglevels = 'CRITICAL;DEBUG;ERROR;FATAL;INFO;NOTSET;WARN;WARNING'.split(';')

    parser = argparse.ArgumentParser(description=__doc__.strip())
    parser.add_argument('src', help='Emoji Sequence Data for UTS #51 file')
    parser.add_argument('dst', help='binary output file')
    parser.add_argument('--overwrite-dst', action='store_true', help='overwrite binary output file, if it already exists')
    parser.add_argument('--log-level', dest='loglevel', default='DEBUG', choices=loglevels, help='loglevel for the logging module')
    parser.add_argument('--log-format', dest='logformat', default='%(asctime)s,%(levelname)s: %(message)s', help='log message for the logging module')

    args = parser.parse_args()
    setup(loglevel=getattr(logging, args.loglevel), logfmt=args.logformat)
    LOGGER.debug("start at {} UTC".format(datetime.datetime.utcnow().isoformat()))
    exitcode = main(args.src, args.dst, args.overwrite_dst) or 0
    LOGGER.debug("end at {} UTC".format(datetime.datetime.utcnow().isoformat()))
    sys.exit(exitcode)
