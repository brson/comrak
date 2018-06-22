import os
import sys

# Use the environment to configure how to find the dylib
root_dir = os.environ.get("SM_DIR")
target = os.environ.get("SM_TARGET")
if not root_dir:
    root_dir = "."
if not target:
    target = "debug"

target_dir = "{}/target/{}".format(root_dir, target)
mac_dylib = "{}/libsnoomark.dylib".format(target_dir)
linux_dylib = "{}/libsnoomark.so".format(target_dir)

if os.path.exists(mac_dylib):
    dylib = mac_dylib
elif os.path.exists(linux_dylib):
    dylib = linux_dylib
else:
    raise Exception("Unable to find snoomark dylib")

# Symlink the snoomark dylib to snoomark.so and load the module
snoomark_so = "{}/snoomark.so".format(target_dir)
if os.path.lexists(snoomark_so):
    os.unlink(snoomark_so)

os.symlink(os.path.abspath(dylib), snoomark_so)
sys.path.append(target_dir)