#pragma once

#include <stdexcept>
#include "hidapi.h"

namespace DeviceID {
    constexpr unsigned short VXE_VID = 0x373b;
    constexpr unsigned short MADR_WIRED_PID = 0x103f;
    constexpr unsigned short MADR_WIRELESS_PID = 0x1040;
} // namespace DeviceID

class Device {
private:
    bool wired;
    hid_device* hid;

    void initialize_hid_device() {
        struct hid_device_info *devs, *cur_dev;
        devs = hid_enumerate(DeviceID::VXE_VID, 0x0);
        cur_dev = devs;
        hid_device *device = nullptr;

        while (cur_dev) {
            if ((cur_dev->product_id == DeviceID::MADR_WIRED_PID || cur_dev->product_id == DeviceID::MADR_WIRELESS_PID)
                && cur_dev->interface_number == 1) {
                device = hid_open_path(cur_dev->path);
                
                if (device){
                    this->wired = (cur_dev->product_id == DeviceID::MADR_WIRED_PID);
                    break;
                }
            }

            cur_dev = cur_dev->next;
        }

        hid_free_enumeration(devs);
        this->hid = device;
    }
public:
    Device() : wired(false), hid(nullptr) {
        if (hid_init() == -1) {
            throw std::runtime_error("Failed to initialize HIDAPI.");
        }   

        initialize_hid_device();

        if (this->hid == nullptr) {
            hid_exit();
            throw std::runtime_error("No compatible device found on Interface 1.");
        }
    }

    ~Device() {
        if (this->hid != nullptr) {
            hid_close(this->hid);
        }

        hid_exit();
    }

    bool is_wired() const {
        return this->wired;
    }

    hid_device* get_hid_device() const {
        return this->hid;
    }
};