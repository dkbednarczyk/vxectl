#pragma once

#include <CLI/CLI.hpp>
#include "device.hpp"

namespace SetCommand {
    struct SetOptions {
        int polling_rate;
        int dpi_stage;
    };

    void setup(CLI::App &app, Device &device);
} // namespace SetCommand

