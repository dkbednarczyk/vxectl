#include <hidapi/hidapi.h>
#include <CLI/CLI.hpp>
#include "set.hpp"
#include "device.hpp"

int main(int argc, char ** argv) {
    Device device;

    CLI::App app{"vxectl - Control your VXE gaming mouse from the command line"};
    app.require_subcommand();

    SetCommand::setup(app, device);

    CLI11_PARSE(app, argc, argv);

    return 0;
}