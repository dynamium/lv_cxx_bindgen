#include "expected.hpp"
#include "optional.hpp"
#include "lvgl/lvgl.h"

namespace lvgl {
    using nonstd::optional;
    using nonstd::expected;

    class Widget {
        private:
            lv_obj_t *object;

        public:
            Widget(lv_obj_t *parent) {
                lv_obj_create(parent);
            }
            Widget(lvgl::Widget &parent) {
                lv_obj_create(parent.get_obj());
            }

            ~Widget() {
                lv_obj_delete(this->object);
            }

            /// NEVER use this function directly, unless STRICTLY NESESSARY!
            lv_obj_t *get_obj() {
                return this->object;
            }

            // MARKER: WIDGET_CLASS_PUBLIC_MEMBERS
    };

    // MARKER: TYPES

    // MARKER: NAMESPACES

    // MARKER: CLASSES
}
