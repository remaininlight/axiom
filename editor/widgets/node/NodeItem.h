#pragma once

#include <QtWidgets/QGraphicsObject>

#include "editor/model/Hookable.h"

namespace AxiomModel {
    class Node;

    class Control;
}

namespace AxiomGui {

    class NodeSurfaceCanvas;

    class NodeItem : public QGraphicsObject, public AxiomModel::Hookable {
    Q_OBJECT

    public:
        NodeSurfaceCanvas *canvas;

        AxiomModel::Node *node;

        NodeItem(AxiomModel::Node *node, NodeSurfaceCanvas *canvas);

        QRectF boundingRect() const override;

        QRectF drawBoundingRect() const;

        void paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget) override;

        QPainterPath shape() const override;

    protected:

        void mouseMoveEvent(QGraphicsSceneMouseEvent *event) override;

        void mousePressEvent(QGraphicsSceneMouseEvent *event) override;

        void mouseReleaseEvent(QGraphicsSceneMouseEvent *event) override;

        void mouseDoubleClickEvent(QGraphicsSceneMouseEvent *event) override;

        void contextMenuEvent(QGraphicsSceneContextMenuEvent *event) override;

    private slots:

        void setPos(QPoint newPos);

        void setSize(QSize newSize);

        void addControl(AxiomModel::Control *control);

        void setIsSelected(bool selected);

        void remove();

        void resizerChanged(QPointF topLeft, QPointF bottomRight);

        void resizerStartDrag();

        void triggerUpdate();

        void triggerGeometryChange();

    signals:

        void resizerPosChanged(QPointF newPos);

        void resizerSizeChanged(QSizeF newSize);

    private:
        bool isDragging = false;
        QPoint mouseStartPoint;

        static const int textOffset = 15;
    };

}
