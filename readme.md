# pick-json-py
python bindings for https://github.com/GerardRodes/pick-json

## usage
```shell
$ cargo build --release
```
python lib will be at `target/release/libpick_json_py.so` you can copy this to your project along a file named `libpick_json_py.py`  with the following content
```py
def __bootstrap__():
   global __bootstrap__, __loader__, __file__
   import sys, pkg_resources, imp
   __file__ = pkg_resources.resource_filename(__name__, 'libpick_json_py.so')
   __loader__ = None; del __bootstrap__, __loader__
   imp.load_dynamic(__name__,__file__)
__bootstrap__()
```

and you will be able of import it like this
```py
from libpick_json_py import pick_json

item_id = pick_json('data.json', 'item_id')
```