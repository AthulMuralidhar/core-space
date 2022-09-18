// the main idea of the project is to port nasa's astrobee into rust
// this is aimed as a prototype for spacecraft core which will go into the space crafts themselves
// so that they can be deployed in space
// main reference repo here: https://github.com/nasa/astrobee
//main doc guide is here: https://nasa.github.io/astrobee/html/astrobee.html#autotoc_md513
// also, ros has a pure rust implementation here: https://github.com/adnanademovic/rosrust

// NOTE:
// static: A possibly mutable variable with 'static lifetime. The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is unsafe.
// ASTROBEE_RESOURCE_DIR: An absolute path to all non-LUA system resources that are used by the nodes in the system. For example, this includes sparse maps, zone files, clutter maps, etc.
static ASTROBEE_RESOURCE_DIR: &str = "";
// ASTROBEE_CONFIG_DIR: An absolute path to all LUA config files that store input parameters for the nodes in the system.
static ASTROBEE_CONFIG_DIR: &str = "";
// ASTROBEE_ROBOT: The class of robot. There must be a LUA file at location $ASTROBEE_CONFIG_DIR/robots/$ASTROBEE_ROBOT.config. This file defines intrinsics, extrinsics, hardware serial numbers and calibration info.
static ASTROBEE_ROBOT: &str = "";
// ASTROBEE_WORLD: The world in which we are operating. There must be a LUA file at location $ASTROBEE_CONFIG_DIR/worlds/$ASTROBEE_WORLD.config. This file defines simulation parameters, zone files, ground truth calibration data, etc.

static ASTROBEE_WORLD: &str = "";
// ASTROBEE_NODEGRAPH: If set, this environment variables configures all ff_nodelets to be spawned in standalone mode. This mode results in a more easy to read node and topic structure that.
static ASTROBEE_NODEGRAPH: &str = "";

fn main() {
    println!("Hello, world!");
}
