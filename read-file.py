import sys
import json
import time

start = time.time()

xy_count = 0
z_count = 0
for line in sys.stdin:
    data = json.loads(line)

    if data['x'] > 0:
        if data['y'] % data['x'] == 0:
            xy_count += 1
    if data['z']:
        z_count += 1

end = time.time()


print(f'xy: {xy_count}')
print(f'z: {z_count}')
print(f'Elapsed: {end - start}s')
