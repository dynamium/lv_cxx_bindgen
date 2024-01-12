#include "expected.hpp"
#include "optional.hpp"
#include <lvgl/lvgl.h>

namespace lvgl {
    using nonstd::optional;
    using nonstd::expected;

    class obj {
        private:
            lv_obj_t *object;

        public:
            obj(lv_obj_t parent) {
                lv_obj_create(parent);
            }
            obj(lvgl::obj &parent) {
                lv_obj_create(parent.get_obj());
            }

            ~obj() {
                lv_obj_delete(this->object);
            }

            /// NEVER use this function directly, unless STRICTLY NESESSARY!
            lv_obj_t *get_obj() {
                return this->object;
            }

            // MARKER: OBJ_CLASS_MEMBERS
    };

    // MARKER: CHILD_NAMESPACES

    // MARKER: CHILD_CLASSES
}
