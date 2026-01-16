#include "polling_rate.hpp"
#include <stdexcept>

namespace PollingRate {
    std::vector<uint8_t> get_magic_packet(int rate) {
        uint8_t rate_byte;
        switch (rate) {
            case 125:
                rate_byte = 0x08;
                break;
            case 250:
                rate_byte = 0x04;
                break;
            case 500:
                rate_byte = 0x02;
                break;
            case 1000:
                rate_byte = 0x01;
                break;

            // wireless only
            case 2000:
                rate_byte = 0x10;
                break;
            case 4000:
                rate_byte = 0x20;
                break;
            case 8000:
                rate_byte = 0x40;
                break;
            default:
                throw std::out_of_range("Invalid polling rate. Valid values are 125, 250, 500, 1000, 2000, 4000, 8000 Hz.");
        }

        return {
            0x08,
            0x07,
            0x00,
            0x00,
            0x00,
            0x06,

            // set polling rate byte
            rate_byte,
            static_cast<uint8_t>(0x55-rate_byte),

            0x04,
            0x51,
            0x1,
            0x54,
            0x00,
            0x00,
            0x00,
            0x00,
            0x41,
        };
    }
}