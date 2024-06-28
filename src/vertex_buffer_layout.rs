use gl;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum VertexBufferLayoutType {
    F32 = gl::FLOAT,
    U32 = gl::UNSIGNED_INT,
}

pub struct VertexBufferLayoutElement {
    pub layout_type: VertexBufferLayoutType,
    pub count: u32,
    pub normalized: bool,
    pub offset: u32,
}

impl VertexBufferLayoutElement {
    fn size_of_layout_type(&self) -> u32 {
        match self.layout_type {
            VertexBufferLayoutType::F32 => std::mem::size_of::<f32>() as u32,
            VertexBufferLayoutType::U32 => std::mem::size_of::<u32>() as u32,
        }
    }
}

#[derive(Default)]
pub struct VertexBufferLayout {
    layouts: Vec<VertexBufferLayoutElement>,
    stride: u32,
}

impl VertexBufferLayout {
    pub fn add(&mut self, layout_type: VertexBufferLayoutType, count: u32, normalized: bool) {
        let layout_element = VertexBufferLayoutElement {
            layout_type,
            count,
            normalized,
            offset: self.stride,
        };

        self.stride += count * layout_element.size_of_layout_type();
        self.layouts.push(layout_element);
    }

    pub fn get_stride(&self) -> u32 {
        self.stride
    }

    pub fn get_layouts(&self) -> &Vec<VertexBufferLayoutElement> {
        &self.layouts
    }
}
