#ifndef SHAPES_H
#define SHAPES_H

#include <GL/glew.h>


class BasicShapeElements
{
public:
    BasicShapeElements(const GLfloat* data, GLsizeiptr byteSize, const GLubyte* indexes, GLsizeiptr indexesByteSize);    
    ~BasicShapeElements();
    
    void enableAttribute(GLuint index, GLint size, GLsizei stride, GLsizeiptr offset);    
    void draw(GLenum mode, GLsizei count);
    
private:
    GLuint m_vao;
    GLuint m_vbo;
    GLuint m_ebo;
};

#endif // SHAPES_H
