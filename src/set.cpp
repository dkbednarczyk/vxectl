#include <iostream>
#include "set.hpp"
#include "CLI/CLI.hpp"
#include "dpi_stage.hpp"
#include "polling_rate.hpp"
#include "hidapi.h"

namespace SetCommand {
    void dpi_stage(Device &device, SetOptions const &opt) {
        auto packet = DPIStage::get_magic_packet(opt.dpi_stage);
        if (hid_write(device.get_hid_device(), packet.data(), packet.size()) == -1) {
            std::cout << "Failed to send DPI command: " << hid_error(device.get_hid_device()) << std::endl;
        } else {
            std::cout << "Set DPI stage to " << (int)opt.dpi_stage << std::endl;
        }
    }

    void polling_rate(Device &device, SetOptions const &opt) {
        auto packet = PollingRate::get_magic_packet(opt.polling_rate);
        if (hid_write(device.get_hid_device(), packet.data(), packet.size()) == -1) {
            std::cout << "Failed to send polling rate command: " << hid_error(device.get_hid_device()) << std::endl;
        } else {
            std::cout << "Set polling rate to " << opt.polling_rate << " Hz" << std::endl;
        }
    }

    void setup(CLI::App &app, Device &device) {
        auto opt = std::make_shared<SetOptions>();
        auto set_cmd = app.add_subcommand("set", "Set device parameters");

        auto dpi_opt = set_cmd->add_option("-s,--dpi-stage", opt->dpi_stage, "DPI stage to enable");
        auto poll_opt = set_cmd
            ->add_option("-p,--polling-rate", opt->polling_rate, "Polling rate to set (125, 250, 500, 1000, 2000, 4000, 8000 Hz)")
            ->check(CLI::IsMember({125, 250, 500, 1000, 2000, 4000, 8000}));

        set_cmd->callback([&device, opt, dpi_opt, poll_opt]() {
            if (dpi_opt->count() > 0) {
                dpi_stage(device, *opt);
            }

            if (poll_opt->count() > 0) {
                if (device.is_wired() && opt->polling_rate > 1000) {
                    std::cout << "Wired mouse only supports up to 1000 Hz polling rate." << std::endl;
                    return;
                }

                polling_rate(device, *opt);
            }
        });
    }
}
