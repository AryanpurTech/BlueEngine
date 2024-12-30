/// A container for uniform buffer types
pub mod uniform_type {

    /// 4 by 4, 32 bit float matrix uniform buffer
    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct Matrix {
        /// data structure for the matrix
        pub data: [[f32; 4]; 4],
    }
    impl Matrix {
        /// Replaces it's values by the new values provided
        pub fn update(&mut self, uniform: Matrix) {
            self.data = uniform.data;
        }

        /// Converts internal matrix to the uniform matrix
        pub fn from_im(matrix: nalgebra_glm::Mat4) -> Self {
            let mtx = matrix.as_slice();

            Self {
                data: [
                    [mtx[0], mtx[1], mtx[2], mtx[3]],
                    [mtx[4], mtx[5], mtx[6], mtx[7]],
                    [mtx[8], mtx[9], mtx[10], mtx[11]],
                    [mtx[12], mtx[13], mtx[14], mtx[15]],
                ],
            }
        }

        /// Converts uniform matrix to internal matrix
        pub fn to_im(&self) -> nalgebra_glm::Mat4 {
            let mtx = self.data;

            nalgebra_glm::mat4(
                mtx[0][0], mtx[0][1], mtx[0][2], mtx[0][3], mtx[1][0], mtx[1][1], mtx[1][2],
                mtx[1][3], mtx[2][0], mtx[2][1], mtx[2][2], mtx[2][3], mtx[3][0], mtx[3][1],
                mtx[3][2], mtx[3][3],
            )
        }
    }
    impl std::ops::Mul for Matrix {
        type Output = Matrix;

        fn mul(self, rhs: Self) -> Self::Output {
            let a = self.data;
            let b = rhs.data;
            Matrix {
                data: [
                    [
                        a[0][0] * b[0][0],
                        a[0][1] * b[1][0],
                        a[0][2] * b[2][0],
                        a[0][3] * b[3][0],
                    ],
                    [
                        a[1][0] * b[0][1],
                        a[1][1] * b[1][1],
                        a[1][2] * b[2][1],
                        a[1][3] * b[3][1],
                    ],
                    [
                        a[2][0] * b[0][2],
                        a[2][1] * b[1][2],
                        a[2][2] * b[2][2],
                        a[2][3] * b[3][2],
                    ],
                    [
                        a[3][0] * b[0][3],
                        a[3][1] * b[1][3],
                        a[3][2] * b[2][3],
                        a[3][3] * b[3][3],
                    ],
                ],
            }
        }
    }

    /// An array with length 3, each 32 bit float value, uniform buffer
    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct Array3 {
        /// data structure for the array
        pub data: [f32; 3],
    }
    impl std::ops::Mul<Array3> for Array3 {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self {
                data: [
                    self.data[0] * rhs.data[0],
                    self.data[1] * rhs.data[1],
                    self.data[2] * rhs.data[2],
                ],
            }
        }
    }
    impl std::ops::Mul<f32> for Array3 {
        type Output = Self;

        fn mul(self, rhs: f32) -> Self::Output {
            Self {
                data: [self.data[0] * rhs, self.data[1] * rhs, self.data[2] * rhs],
            }
        }
    }

    /// An array with length 4, each 32 bit float value, uniform buffer
    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct Array4 {
        /// data structure for the array
        pub data: [f32; 4],
    }
    impl Array4 {
        /// Replaces it's values by the new values provided
        pub fn update(&mut self, uniform: Array4) {
            self.data = uniform.data;
        }
    }
    impl std::ops::Mul<Array4> for Array4 {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Array4 {
                data: [
                    self.data[0] * rhs.data[0],
                    self.data[1] * rhs.data[1],
                    self.data[2] * rhs.data[2],
                    self.data[3] * rhs.data[3],
                ],
            }
        }
    }
    impl std::ops::Mul<f32> for Array4 {
        type Output = Array4;

        fn mul(self, rhs: f32) -> Self::Output {
            Array4 {
                data: [
                    self.data[0] * rhs,
                    self.data[1] * rhs,
                    self.data[2] * rhs,
                    self.data[3] * rhs,
                ],
            }
        }
    }

    /// A 32 bit float uniform buffer
    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct Float {
        /// data structure for the float
        pub data: f32,
    }
    impl Float {
        /// Replaces it's values by the new values provided
        pub fn update(&mut self, uniform: Float) {
            self.data = uniform.data;
        }
    }
}
